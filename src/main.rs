mod server;
use server::Server;

mod handler_web;
use handler_web::HandlerWeb;

mod http;

fn main() {
    let app = Server::new("127.0.0.1".to_string(), 8080);
    app.run(HandlerWeb);
}
