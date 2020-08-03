use diesel::prelude::*;
use diesel::PgConnection;
use std::collections::HashMap;

use crate::grid::Coords;
use crate::schema::*;

const MAX_AMOUNT: i32 = 5000;

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
    pub fn load_all(conn: Option<&PgConnection>) -> Result<HashMap<i64, Valuable>, String> {
        if conn.is_none() {
            return Err("No DB connection".to_string());
        }

        let mut _valuables = HashMap::new();
        let results = valuables::table
            .load::<Valuable>(conn.unwrap())
            .expect("Failed to load robots");

        for result in results {
            let id = result.id;

            _valuables.insert(id, result);
        }

        Ok(_valuables)
    }

    /// persist current values to the db
    fn persist_to_db(&mut self, conn: &PgConnection) {
        // update the db
        let _ = diesel::update(valuables::table.filter(valuables::id.eq(self.id)))
            .set(valuables::amount.eq(self.amount))
            .execute(conn);
    }

    /// Increase in value
    pub fn add_to_amount(&mut self, conn: Option<&PgConnection>, amount: i32) {
        self.amount += amount;
        if self.amount > MAX_AMOUNT {
            self.amount = MAX_AMOUNT;
        }

        if conn.is_some() {
            self.persist_to_db(conn.unwrap());
        }
    }

    /// Attempt to mine a certain amount
    pub fn mine(&mut self, conn: Option<&PgConnection>, amount: i32) -> i32 {
        let mined_amount: i32;

        if self.amount < amount {
            mined_amount = self.amount;
            self.amount = 0;
        } else {
            mined_amount = amount;
            self.amount -= mined_amount;
        }

        if conn.is_some() {
            self.persist_to_db(conn.unwrap());
        }

        mined_amount
    }

    /// Delete self
    pub fn destroy(&mut self, conn: Option<&PgConnection>) -> bool {
        println!("Valuable {}: Destroy", self.id);
        if conn.is_some() {
            let _ = diesel::delete(valuables::table.filter(valuables::id.eq(self.id)))
                .execute(conn.unwrap());
        }

        true
    }
}
