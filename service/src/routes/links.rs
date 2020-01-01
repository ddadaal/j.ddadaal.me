use crate::data::repositories::jumps::JumpsRepository;
use crate::data::repositories::stats::StatsRepository;
use crate::Pool;
use actix_web::HttpResponse;
use actix_web::{dev, get, http, web, Responder};
use chrono;
use diesel::prelude::*;
use futures::future::FutureExt;
use log::{error, info};
use std::sync::Arc;

/// The redirection
#[get("/{from}")]
pub async fn get_link(
    info: web::Path<String>,
    jumps_repo: web::Data<Arc<dyn JumpsRepository>>,
    stats_repo: web::Data<Arc<dyn StatsRepository>>,
    req: web::HttpRequest,
) -> impl Responder {
    let from = info.into_inner();

    info!("Jump requested: {}", from);

    let target = jumps_repo.get_jump_link(from.clone()).await;

    match target {
        Ok(result) => match result {
            Some(target) => {
                info!("Jump successful: {} => {}", from, target);

                // There is no need to await,
                // but I could not make it work
                // Would try again when async {} is stablized.
                if let Err(err) = stats_repo
                    .add_access(
                        req.connection_info().host().into(),
                        from,
                        req.uri().to_string(),
                        chrono::Local::now().naive_utc(),
                    )
                    .await
                {
                    error!("Error when adding stats: {}", err);
                }

                // The following doesn't work
                // for return a ref to stats_repo
                // which have been moved into the closure
                // actix_rt::Arbiter::spawn_fn(move || {
                //     // let stats_repo = stats_repo.get_ref().clone();
                //     stats_repo
                //         .add_access(
                //             req.connection_info().host().into(),
                //             from,
                //             req.uri().to_string(),
                //             chrono::Local::now().naive_utc(),
                //         )
                //         .map(|x| {
                //             if let Err(err) = x {
                //                 error!("Error when adding stat: {}", err);
                //             }
                //             ()
                //         })
                // });
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

#[cfg(test)]
mod test {

    use super::*;
    use actix_web::{test, HttpMessage, HttpRequest, HttpResponse};

    #[test]
    fn test() {
        // let req = test::TestRequest::get().uri("/123").app_data(data: T)
    }
}
