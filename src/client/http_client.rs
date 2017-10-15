extern crate hyper;
extern crate tokio_core;
extern crate pretty_env_logger;
extern crate futures;

use hyper::Client;
use hyper::client::HttpConnector;

struct HttpClient {
    core: tokio_core::reactor::Core,
    handle: tokio_core::reactor::Handle,
}

impl HttpClient {
    pub fn new() -> HttpClient {
        let mut _core = tokio_core::reactor::Core::new().unwrap();
        let _handle   = _core.handle();
        HttpClient {
            core: _core,
            handle: _handle,
        }
    }

    pub fn connect(&self) -> Client<HttpConnector> {
        Client::new(&self.handle)
    }
}

fn main() {
    pretty_env_logger::init().unwrap();

    let http_client = HttpClient::new();
    let client = http_client.connect();

    
    println!("Http client is initialized");
}
