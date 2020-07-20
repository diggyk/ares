use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;

pub struct DbConfig {
    pub dbuser: String,
    pub dbpw: String,
    pub dbhost: String,
    pub dbname: String,
}

impl DbConfig {
    pub fn to_url(&self) -> String {
        format!(
            "postgres://{}:{}@{}/{}",
            self.dbuser, self.dbpw, self.dbhost, self.dbname
        )
    }
}

pub fn establish_connection(dbconfig: &DbConfig) -> PgConnection {
    dotenv().ok();

    let database_url = dbconfig.to_url();
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}
