use clap::{App, Arg};
use diesel::pg::PgConnection;

pub mod server;
pub use server::Server;

use crate::db::*;

pub struct ServerConfig {
    pub dbconfig: DbConfig,
    pub conn: PgConnection,

    // maximum number of robots to spawn
    max_bots: usize,
    // how quickly to regen robots
    // regen_rate: usize,
} 

pub fn get_config() -> ServerConfig {
    let matches = App::new("Ares Grid Admin")
        .version("0.1.0")
        .about("Create/maintain grids")
        .arg(Arg::with_name("dbuser")
            .short("u")
            .long("user")
            .takes_value(true)
            .help("Database username"))
        .arg(Arg::with_name("dbpw")
            .short("p")
            .long("password")
            .takes_value(true)
            .help("Database password"))
        .arg(Arg::with_name("dbhost")
            .short("o")
            .long("hostname")
            .takes_value(true)
            .help("Database hostname"))
        .arg(Arg::with_name("db")
            .short("n")
            .long("dbname")
            .takes_value(true)
            .help("Database name"))
        .arg(Arg::with_name("max_bots")
            .required(true)
            .takes_value(true)
            .help("Maximum number of robots"))
        .arg(Arg::with_name("regen_rate")
            .required(true)
            .takes_value(true)
            .help("How often to spawn a robot (in seconds)"))
        .get_matches();

        let dbuser = matches.value_of("dbuser").unwrap_or("ares").to_string();
        let dbpw = matches.value_of("dbpw").unwrap_or("ares").to_string();
        let dbhost = matches.value_of("dbhost").unwrap_or("localhost").to_string();
        let dbname = matches.value_of("db").unwrap_or("ares").to_string();

        let max_bots = matches.value_of("max_bots").unwrap_or("10");
        let max_bots = max_bots.parse::<usize>().expect("Could not parse max bots");

        // let regen_rate = matches.value_of("regen_rate").unwrap_or("1");
        // let regen_rate = regen_rate.parse::<usize>().expect("Could not parse regen rate");

        let dbconfig = DbConfig{dbuser, dbpw, dbhost, dbname};
        let conn = establish_connection(&dbconfig);

        ServerConfig {
            dbconfig, conn, max_bots,
        }
}