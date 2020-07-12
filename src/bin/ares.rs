use ares::server;

fn main() {
    let config = server::get_config();
    let server = server::Server::new(config);

    server.run();
}