use ares::server;
use ares::db;

fn main() {
    let config = server::get_config();
    
    let connection = db::establish_connection(&config);

    ctrlc::set_handler(move || {
        println!("Signal for shutdown");
        std::process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");

    server.run();
}