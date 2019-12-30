use crate::Pool;
use actix_web::HttpResponse;
use actix_web::{get, http, web, Responder};
use diesel::prelude::*;
use tokio_diesel::{OptionalExtension, *};

/// The redirection
#[get("/{short_link}")]
pub async fn get_link(info: web::Path<String>, pool: web::Data<Pool>) -> impl Responder {
    use crate::db::schema::links;
    let short_link = info.into_inner();

    info!("Jump requested: {}", short_link);

    // SELECT full_link FROM links WHERE short_link = {short_link};
    let full_link = links::table
        .filter(links::short_link.eq(short_link.clone()))
        .limit(1)
        .select(links::full_link)
        .get_result_async::<String>(&pool)
        .await;

    match full_link.optional() {
        Ok(result) => match result {
            Some(full_link) => {
                info!("Jump successful: {} => {}", short_link, full_link);
                HttpResponse::MovedPermanently()
                    .header(http::header::LOCATION, full_link)
                    .finish()
            }
            None => {
                info!("No jump: {}", short_link);
                HttpResponse::NotFound().finish()
            }
        },
        Err(err) => {
            error!("500 when processing /{}: {}", short_link, err);
            HttpResponse::InternalServerError().finish()
        }
    }
}
