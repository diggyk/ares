use diesel::prelude::*;
use diesel::PgConnection;
use std::collections::HashMap;

use crate::grid::Coords;
use crate::schema::*;

#[derive(Debug, Queryable, Insertable)]
#[table_name = "valuables"]
pub struct NewValuable {
    pub q: i32,
    pub r: i32,
    pub kind: String,
    pub amount: i32,
}

#[derive(Debug, Queryable, Identifiable, Insertable)]
#[table_name = "valuables"]
pub struct Valuable {
    pub id: i64,
    pub q: i32,
    pub r: i32,
    pub kind: String,
    pub amount: i32,
}

impl Valuable {
    pub fn new(coords: Coords, amount: i32, conn: Option<&PgConnection>) -> Valuable {
        let new_valuable = NewValuable {
            q: coords.q,
            r: coords.r,
            kind: String::from("basic"),
            amount,
        };

        let mut _valuable: Valuable;
        if let Some(conn) = conn {
            _valuable = diesel::insert_into(valuables::table)
                .values(new_valuable)
                .get_result(conn)
                .expect("Error saving cells");
        } else {
            _valuable = Valuable {
                id: 0,
                q: coords.q,
                r: coords.r,
                kind: String::from("basic"),
                amount,
            }
        }

        _valuable
    }

    /// Load all the robots out of the database
    pub fn load_all(conn: &PgConnection) -> HashMap<i64, Valuable> {
        let mut _valuables = HashMap::new();
        let results = valuables::table
            .load::<Valuable>(conn)
            .expect("Failed to load robots");

        for result in results {
            let id = result.id;

            _valuables.insert(id, result);
        }

        _valuables
    }
}
