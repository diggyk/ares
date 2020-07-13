use ares::server;

fn main() {
    let config = server::get_config();
    let mut server = server::Server::new(config);

    ctrlc::set_handler(move || {
        println!("Signal for shutdown");
        std::process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");
    
    server.run();
}