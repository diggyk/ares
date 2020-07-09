use postgres::{Client, NoTls, Error};
use std::collections::HashMap;

use crate::grid::Coords;
use crate::grid::GridCell;

#[derive(Debug)]
pub struct DbClient {
    user: String,
    pw: String,
    host: String,
    db: String,
}

impl DbClient {
    pub fn new(user: &str, pw: &str, host: &str, db: &str) -> DbClient {
        DbClient {
            user: user.to_string(),
            pw: pw.to_string(),
            host: host.to_string(),
            db: db.to_string(),
        }
    }

    pub fn drop_all_cells(&self) {
        /* Drops all the cells from the DB */
        let connstr = format!("postgresql://{}:{}@{}/{}", self.user, self.pw, self.host, self.db);
        let mut client = Client::connect(&connstr, NoTls).expect(
            &String::from("Failed to connect to DB")
        );
    
        client.execute("TRUNCATE gridcells", &[]).expect(&String::from("Could not truncate gridcells"));
    }
    
    pub fn create_cells(&self, cells: &HashMap<Coords, GridCell>) {
        /* Create's the cells given in the gridcell table */
        let connstr = format!("postgresql://{}:{}@{}/{}", self.user, self.pw, self.host, self.db);
        let mut client = Client::connect(&connstr, NoTls).expect(
            &String::from("Failed to connect to DB")
        );
    
        let stmt = client.prepare("INSERT INTO gridcells(id, q, r) VALUES ($1, $2, $3)").expect(
            &String::from("Failed to prepare statement")
        );
    
        let mut count = 0;
        let total = cells.len();
    
        for (coords, cell) in cells {
            if let Err(error) = client.execute(
                &stmt,
                &[&cell.id, &coords.q, &coords.r], 
            ) {
                println!("Failed to insert grid cell: {:?}", error);
            }
    
            count += 1;
            if count % 500 == 0 {
                println!("Insert cells: {}%  {}/{}", (count as f32/total as f32 * 100.0), count, total);
            }
        }
    }
}