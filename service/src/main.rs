mod db;
mod routes;

#[macro_use]
extern crate diesel;

use actix_web::{middleware, App, HttpServer};
use diesel::prelude::SqliteConnection;
use diesel::r2d2::{self, ConnectionManager};

type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let database_url = std::env::var("DATABASE_URL").expect("No DATABASE_URL is set.");

    let connection_manager = ConnectionManager::<SqliteConnection>::new(database_url);

    let pool = r2d2::Pool::builder()
        .build(connection_manager)
        .expect("Failed to create pool.");

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            // get /{short_link}
            .service(routes::links::get_link)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
