mod server;

use server::http_server::HttpServer;
use futures::executor;
fn main() {
    let http_server = HttpServer::new("127.0.0.1:3000".parse().unwrap());
    http_server.display();
    executor::block_on(http_server.run());
}