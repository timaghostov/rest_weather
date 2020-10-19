
#![allow(unused_variables, unused_imports, dead_code)]


extern crate serde;
extern crate serde_json;
extern crate serde_derive;
extern crate toml;
extern crate chrono;
extern crate reqwest;


mod model;
mod service;
mod rest;
// mod app;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    
    rest::RestServer::run().await

}