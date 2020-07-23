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

    /// Attempt to mine a certain amount
    pub fn mine(&mut self, conn: &PgConnection, amount: i32) -> i32 {
        let mined_amount: i32;

        if self.amount < amount {
            mined_amount = self.amount;
            self.amount = 0;
        } else {
            mined_amount = amount;
            self.amount -= mined_amount;
        }

        // update the db
        let _ = diesel::update(valuables::table.filter(valuables::id.eq(self.id)))
            .set(valuables::amount.eq(self.amount))
            .execute(conn);

        println!(
            "Valuable {}: mined {} ({} left)",
            self.id, mined_amount, self.amount
        );
        mined_amount
    }

    /// Delete self
    pub fn destroy(&mut self, conn: &PgConnection) -> bool {
        println!("Valuable {}: Destroy", self.id);
        let _ = diesel::delete(valuables::table.filter(valuables::id.eq(self.id))).execute(conn);

        true
    }
}
