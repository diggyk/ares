use clap::{Arg, App};

use ares::grid::Grid;
use ares::db;

fn main() {
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
        .arg(Arg::with_name("size")
            .required(true)
            .takes_value(true)
            .help("Grid radius size"))
        .get_matches();

    let dbuser = matches.value_of("dbuser").unwrap_or("ares").to_string();
    let dbpw = matches.value_of("dbpw").unwrap_or("ares").to_string();
    let dbhost = matches.value_of("dbhost").unwrap_or("localhost").to_string();
    let dbname = matches.value_of("db").unwrap_or("ares").to_string();

    let size = matches.value_of("size").unwrap_or("100");
    let size = size.parse::<u32>().expect("Could not parse size");

    let dbconfig = db::DbConfig{dbuser, dbpw, dbhost, dbname};
    let connection = db::establish_connection(&dbconfig);
    
    let grid = Grid::new(size, Some(&connection)).unwrap();
    println!("Cells: {}", grid.cells.len())
}
