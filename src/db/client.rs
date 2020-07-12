use postgres::{Client, NoTls, Error};
use std::collections::HashMap;

use crate::grid::Coords;
use crate::grid::GridCell;
use crate::robot::Robot;

pub struct DbClient {
    user: String,
    pw: String,
    host: String,
    db: String,

    client: Option<postgres::Client>,
}

impl DbClient {
    pub fn new(user: &str, pw: &str, host: &str, db: &str) -> DbClient {
        DbClient {
            user: user.to_string(),
            pw: pw.to_string(),
            host: host.to_string(),
            db: db.to_string(),
            client: None,
        }
    }

    fn connect(&mut self) {
        let connstr = format!("postgresql://{}:{}@{}/{}", self.user, self.pw, self.host, self.db);
        let client = Client::connect(&connstr, NoTls).expect(
            &String::from("Failed to connect to DB")
        );
        self.client = Some(client);
    }

    pub fn drop_all_cells(&mut self) {
        /* Drops all the cells from the DB */
        if let None = self.client {
            self.connect();
        }
        self.client.as_mut().unwrap().execute("TRUNCATE gridcells", &[]).expect(&String::from("Could not truncate gridcells"));
    }
    
    pub fn create_cells(&mut self, cells: &HashMap<Coords, GridCell>) {
        /* Create's the cells given in the gridcell table */
        if let None = self.client {
            self.connect();
        }
    
        let stmt = self.client.as_mut().unwrap().prepare(
            "INSERT INTO gridcells(id, q, r, edge0, edge60, edge120, edge180, edge240, edge300) \
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)"
        ).expect(
            &String::from("Failed to prepare statement")
        );
    
        let mut count = 0;
        let total = cells.len();
    
        for (coords, cell) in cells {
            let edge0: i16 = cell.edge0.clone().into();
            let edge60: i16 = cell.edge60.clone().into();
            let edge120: i16 = cell.edge120.clone().into();
            let edge180: i16 = cell.edge180.clone().into();
            let edge240: i16 = cell.edge240.clone().into();
            let edge300: i16 = cell.edge300.clone().into();

            if let Err(error) = self.client.as_mut().unwrap().execute(
                &stmt,
                &[&cell.id, &coords.q, &coords.r, &edge0, &edge60, &edge120, &edge180, &edge240, &edge300], 
            ) {
                println!("Failed to insert grid cell: {:?}", error);
            }
    
            count += 1;
            if count % 500 == 0 {
                println!("Insert cells: {}%  {}/{}", (count as f32/total as f32 * 100.0), count, total);
            }
        }
    }

    pub fn get_all_cells(&mut self) -> HashMap<Coords, GridCell> {
        if let None = self.client {
            self.connect();
        }

        let mut cells: HashMap<Coords, GridCell> = HashMap::new();

        let results = self.client.as_mut().unwrap().query(
            "SELECT id, q, r, edge0, edge60, edge120, edge180, edge240, edge300 from gridcells", &[],
        );

        if let Ok(results) = results {
            for result in results {
                let cell: GridCell = result.into();
                let coords = cell.coords.clone();
                cells.insert(coords, cell);
            }
        }
        cells
    }

    pub fn create_robot(&mut self, robot: &mut Robot) -> Result<(), String> {
        if let None = self.client {
            self.connect();
        }

        let client = self.client.as_mut().unwrap();
        let orientation: i16 = robot.orientation.clone().into();
        let results = client.query(
            "INSERT INTO robots(name, q, r, orientation) VALUES ($1, $2, $3, $4) RETURNING id",
            &[&robot.name, &robot.coords.q, &robot.coords.r, &orientation]
        );

        if let Ok(results) = results {
            for result in results {
                robot.id = result.get(0);
            }
            Ok(())
        } else {
            Err(String::from("Could not insert"))
        }
    }
}