use crate::http::Request;
use std::{io::Read, net::TcpListener};

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
                Ok((mut s, _)) => {
                    println!("The connection with has been established!");
                    let mut buffer = [0; 1024];
                    match s.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));
                            match Request::try_from(&buffer[..]) {
                                Ok(req) => {
                                    dbg!(req);
                                }
                                Err(e) => {
                                    println!("Failed to parse the contents of request: {}", e)
                                }
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
