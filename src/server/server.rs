#![deny(warnings)]

extern crate futures;
extern crate hyper;
extern crate pretty_env_logger;
extern crate url;

use std::env;

use futures::future::FutureResult;

use hyper::{Get, Post, StatusCode};
use hyper::header::ContentLength;
use hyper::server::{Http, Service, Request, Response};

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

#[derive(Serialize, Deserialize, Debug)]
struct AppResponse {
    count: i32,
    result: String,
}

struct Echo;

impl Service for Echo {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = FutureResult<Response, hyper::Error>;

    fn call(&self, req: Request) -> Self::Future {
        futures::future::ok(match (req.method(), req.path()) {
            (&Get, "/") => {
                let res = "Hi you are index page".to_owned();
                Response::new()
                    .with_header(ContentLength(res.len() as u64))
                    .with_body(res)
            },
            (&Get, "/echo") => {
                let res = AppResponse { count: 10, result: "success".to_owned() };
                let res_str = serde_json::to_string(&res).unwrap();
                Response::new()
                    .with_header(ContentLength(res_str.len() as u64))
                    .with_body(res_str)
            },
            (&Post, "/echo") => {
                println!("Received post request");
                let mut res = Response::new();
                if let Some(len) = req.headers().get::<ContentLength>() {
                    println!("Content length {}", len.clone());
                    res.headers_mut().set(len.clone());
                }
                res.with_body(req.body())
            },
            _ => {
                Response::new()
                    .with_status(StatusCode::NotFound)
            }
        })
    }
}

fn main() {
    pretty_env_logger::init().unwrap();

    let n_args = env::args().len();
    if n_args != 3 {
        println!("Usage: {} <host> <port>", env::args().nth(0).unwrap());
        return;
    }
    // form the address
    let mut host_addr = env::args().nth(1).unwrap();
    host_addr.push_str(":");
    host_addr.push_str(&env::args().nth(2).unwrap());
    let addr = host_addr.parse().unwrap();

    let server = Http::new().bind(&addr, || Ok(Echo)).unwrap();
    println!("Listening on http://{} with 1 thread", server.local_addr().unwrap());
    server.run().unwrap();
}
