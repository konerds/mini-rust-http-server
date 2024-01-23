use crate::http::{Request, Response, StatusHttp};
use std::io::{Read, Write};
use std::net::TcpListener;

pub struct Server {
    hostname: String,
    port: u16,
}

impl Server {
    pub fn new(hostname: String, port: u16) -> Self {
        Server { hostname, port }
    }
    pub fn run(self) {
        println!("Trying to listening on {}:{}", self.hostname, self.port);
        let listener = TcpListener::bind(format!("{0}:{1}", self.hostname, self.port)).unwrap();
        println!("Listening on {}:{}", self.hostname, self.port);
        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    println!("The connection with has been established!");
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));
                            let response = match Request::try_from(&buffer[..]) {
                                Ok(req) => {
                                    dbg!(req);
                                    Response::new(
                                        StatusHttp::Ok,
                                        Some("<h1>Hello World!!!!</h1>".to_string()),
                                    )
                                }
                                Err(e) => {
                                    println!("Failed to parse the request: {}", e);
                                    Response::new(StatusHttp::BadRequest, None)
                                }
                            };
                            if let Err(e) = response.send(&mut stream) {
                                println!("Failed to send response: {}", e);
                            }
                        }
                        Err(e) => println!("Failed to read: {}", e),
                    }
                }
                Err(e) => println!("Failed to establish a connection: {}", e),
            }
        }
    }
}
