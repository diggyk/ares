
use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

pub mod client;

pub use client::DbClient;

pub struct DbConfig {
    dbuser: String,
    dbpw: String,
    dbhost: String,
    dbname: String,
}

impl DbConfig {
    pub fn to_url(&self) -> String {
        format!("postgres://{}:{}@{}/{}", self.dbuser, self.dbpw, self.dbhost, self.dbname)
    }
}

pub fn establish_connection(dbconfig: &DbConfig) -> PgConnection {
    dotenv().ok();

    let database_url = dbconfig.to_url();
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}