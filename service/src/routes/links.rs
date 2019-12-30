use crate::Pool;
use actix_web::HttpResponse;
use actix_web::{get, http, web, Responder};
use diesel::prelude::*;
use tokio_diesel::{OptionalExtension, *};

/// The redirection
#[get("/{from}")]
pub async fn get_link(info: web::Path<String>, pool: web::Data<Pool>) -> impl Responder {
    use crate::db::schema::jumps;
    let from = info.into_inner();

    info!("Jump requested: {}", from);

    // SELECT full_link FROM links WHERE short_link = {short_link};
    let target = jumps::table
        .filter(jumps::from.eq(from.clone()))
        .limit(1)
        .select(jumps::to)
        .get_result_async::<String>(&pool)
        .await;

    match target.optional() {
        Ok(result) => match result {
            Some(target) => {
                info!("Jump successful: {} => {}", from, target);
                HttpResponse::MovedPermanently()
                    .header(http::header::LOCATION, target)
                    .finish()
            }
            None => {
                info!("No jump: {}", from);
                HttpResponse::NotFound().finish()
            }
        },
        Err(err) => {
            error!("500 when processing /{}: {}", from, err);
            HttpResponse::InternalServerError().finish()
        }
    }
}
