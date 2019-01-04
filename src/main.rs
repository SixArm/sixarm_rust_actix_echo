extern crate actix_web;
use actix_web::{server, App, Path, Responder};

fn index(info: Path<String>) -> impl Responder {
    format!("{} ", s)
}

fn main() {
    server::new(|| {
        App::new().resource("/{s}", |r| r.with(index))
    }).bind("127.0.0.1:8000")
        .unwrap()
        .run();
}
