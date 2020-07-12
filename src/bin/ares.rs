use ares::server;

fn main() {
    let config = server::get_config();
    let mut server = server::Server::new(config);

    server.run();
}