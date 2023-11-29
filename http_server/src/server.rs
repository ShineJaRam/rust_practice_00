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
            let res = lisenter.accept();

            if res.is_err() {
                continue;
            }

            let stream = res.unwrap();
        }
    }
}
