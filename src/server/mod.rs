use clap::{App, Arg};
use diesel::pg::PgConnection;

pub mod broadcast;
pub mod server;
pub mod ws;

pub use server::Server;
pub use ws::WebsocketServer;

use crate::db::*;

pub struct ServerConfig {
    pub dbconfig: DbConfig,
    pub conn: Option<PgConnection>,

    // maximum number of robots to spawn
    max_bots: usize,

    // maximum number of valuables files
    max_valuables: usize,

    // do killed robots drop valuables
    no_kill_drops: bool,

    debug: bool,
}

pub fn get_config() -> ServerConfig {
    let matches = App::new("Ares Grid Admin")
        .version("0.1.0")
        .about("Create/maintain grids")
        .arg(
            Arg::with_name("dbuser")
                .short("u")
                .long("user")
                .takes_value(true)
                .help("Database username"),
        )
        .arg(
            Arg::with_name("dbpw")
                .short("p")
                .long("password")
                .takes_value(true)
                .help("Database password"),
        )
        .arg(
            Arg::with_name("dbhost")
                .short("o")
                .long("hostname")
                .takes_value(true)
                .help("Database hostname"),
        )
        .arg(
            Arg::with_name("db")
                .short("n")
                .long("dbname")
                .takes_value(true)
                .help("Database name"),
        )
        .arg(
            Arg::with_name("max_bots")
                .required(true)
                .takes_value(true)
                .help("Maximum number of robots"),
        )
        .arg(
            Arg::with_name("max_valuables")
                .required(true)
                .takes_value(true)
                .help("How many valuables piles to keep"),
        )
        .arg(
            Arg::with_name("no_kill_drops")
                .long("no_kill_drops")
                .help("Kill will not drop valuables"),
        )
        .arg(
            Arg::with_name("debug")
                .long("debug")
                .help("Set to debug mode"),
        )
        .get_matches();

    let dbuser = matches.value_of("dbuser").unwrap_or("ares").to_string();
    let dbpw = matches.value_of("dbpw").unwrap_or("ares").to_string();
    let dbhost = matches
        .value_of("dbhost")
        .unwrap_or("localhost")
        .to_string();
    let dbname = matches.value_of("db").unwrap_or("ares").to_string();

    let max_bots = matches.value_of("max_bots").unwrap_or("10");
    let max_bots = max_bots.parse::<usize>().expect("Could not parse max bots");

    let max_valuables = matches.value_of("max_valuables").unwrap_or("1");
    let max_valuables = max_valuables
        .parse::<usize>()
        .expect("Could not parse max valuables");

    let dbconfig = DbConfig {
        dbuser,
        dbpw,
        dbhost,
        dbname,
    };
    let conn = establish_connection(&dbconfig);

    ServerConfig {
        dbconfig,
        conn: Some(conn),
        max_bots,
        max_valuables,
        no_kill_drops: matches.is_present("no_kill_drops"),
        debug: matches.is_present("debug"),
    }
}

/// With each tick, a robot can make a request of the server
#[derive(Clone, Debug, PartialEq)]
pub enum Request {
    /// Request to attack an enemy
    Attack { target_id: i64 },
    /// Request to leave the grid
    Exfiltrate { robot_id: i64 },
    /// Request to explode, leaving behind valuables
    Explode { valuables: i32 },
    /// Request to mine the valuable for a given amount
    Mine { valuable_id: i64, amount: i32 },
}

/// For each server request, the server can respond to the robot
#[derive(Clone, Debug, PartialEq)]
pub enum Response {
    AttackFailed,
    AttackSuccess { target_id: i64, damage: i32 },
    Fail,
    Mined { valuable_id: i64, amount: i32 },
}
