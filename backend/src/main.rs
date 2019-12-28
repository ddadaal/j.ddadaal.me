mod db;


#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;

#[actix_rt::main]
async fn main() {
    std::env::set_var("RUST_LOG", "actix_web=info");

    println!("Hello, world!");
}
