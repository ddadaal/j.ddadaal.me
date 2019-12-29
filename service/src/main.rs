mod db;
mod routes;

#[macro_use]
extern crate diesel;

#[actix_rt::main]
async fn main() {
    std::env::set_var("RUST_LOG", "actix_web=info");

    println!("Hello, world!");
}
