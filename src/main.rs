mod data;
mod routes;

#[macro_use]
extern crate diesel;

use crate::data::repositories::jumps::{JumpsRepository, JumpsRepositoryImpl};
use crate::data::repositories::stats::{StatsRepository, StatsRepositoryImpl};
use actix_web::{middleware, App, HttpServer};
use diesel::prelude::SqliteConnection;
use diesel::r2d2::{self, ConnectionManager};
use std::sync::Arc;

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

    // create repositories
    let jumps_repo = Arc::new(JumpsRepositoryImpl::new(pool.clone()));

    let stats_repo = Arc::new(StatsRepositoryImpl::new(pool.clone()));

    HttpServer::new(move || {
        App::new()
            .data::<Arc<dyn JumpsRepository>>(jumps_repo.clone())
            .data::<Arc<dyn StatsRepository>>(stats_repo.clone())
            .wrap(middleware::Logger::default())
            // get /{short_link}
            .service(routes::links::get_link)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
