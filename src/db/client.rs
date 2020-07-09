use postgres::{Client, NoTls, Error};
use std::collections::HashMap;

use crate::grid::Coords;
use crate::grid::GridCell;

pub fn create_cells(cells: &HashMap<Coords, GridCell>) {
    let mut client = Client::connect("postgresql://plexms:plexms3000@192.168.1.7/ares", NoTls).expect(
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