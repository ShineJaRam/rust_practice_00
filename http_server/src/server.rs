use crate::http::Request;
use std::convert::TryFrom;
use std::convert::TryInto;
use std::io::Read;
use std::net::TcpListener;

pub struct Server {
    address: String,
}

impl Server {
    pub fn new(address: String) -> Self {
        Self { address }
    }

    pub fn run(self) {
        println!("Listening on {}", self.address);

        let lisenter = TcpListener::bind(&self.address).unwrap();

        loop {
            match lisenter.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));

                            match Request::try_from(&buffer[..]) {
                                Ok(request) => {}
                                Err(e) => println!("Failed to parse a request: {}", e),
                            }
                        }
                        Err(e) => println!("Failed to read from connection: {}", e),
                    };
                }
                Err(e) => {
                    println!("Failed to establish a connection: {}", e);
                }
            }

            let res = lisenter.accept();

            if res.is_err() {
                continue;
            }

            let (stream, addr) = res.unwrap();
        }
    }
}
