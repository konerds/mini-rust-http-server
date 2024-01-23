use std::env;

mod server;
use server::Server;

mod handler_web;
use handler_web::HandlerWeb;

mod http;

fn main() {
    let path_default = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
    let path_public = env::var("PATH_PUBLIC").unwrap_or(path_default);
    let app = Server::new("127.0.0.1".to_string(), 8080);
    app.run(HandlerWeb::new(path_public));
}
