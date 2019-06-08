use harbourmaster::{Container, Protocol};
use std::net::TcpListener;
use tokio::runtime::Runtime;

pub struct CouchDbContainer {
    container: Container,
    host_port: u16,
}

impl Default for CouchDbContainer {
    fn default() -> Self {
        let host_port = get_unused_port();
        let container = Runtime::new()
            .unwrap()
            .block_on(
                Container::builder("couchdb")
                    .pull_on_build(true)
                    .name("couchdb")
                    .slug_length(6)
                    .expose(5984, host_port, Protocol::Tcp)
                    .build(),
            )
            .unwrap();

        Self {
            container,
            host_port,
        }
    }
}

impl CouchDbContainer {
    pub fn port(&self) -> u16 {
        self.host_port
    }

    pub fn delete(self) {
        Runtime::new()
            .unwrap()
            .block_on(self.container.delete())
            .unwrap();
    }
}

fn get_unused_port() -> u16 {
    let listener = TcpListener::bind("localhost:0").unwrap();
    listener.local_addr().unwrap().port()
}
