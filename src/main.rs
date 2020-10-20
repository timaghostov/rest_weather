


extern crate serde;
extern crate serde_json;
extern crate serde_derive;
extern crate toml;
extern crate chrono;
extern crate reqwest;


mod model;
mod service;
mod rest;

#[cfg(test)]
mod tests;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    
    rest::RestServer::run().await

}