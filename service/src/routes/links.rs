use crate::data::repositories::jumps::JumpsRepository;
use crate::data::repositories::stats::StatsRepository;
use actix_web::HttpResponse;
use actix_web::{get, http, web, Responder};
use chrono;
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
    use crate::data::repositories::jumps::mock::JumpsRepositoryMock;
    use crate::data::repositories::stats::mock::StatsRepositoryMock;
    use actix_service::Service;
    use actix_web::{test, App};

    // can not extract duplicate code
    // 1. extract get_response(url: String) -> ServerResponse
    //    not possible for ServerResponse is private
    // 2. extract get_app() -> impl actix_service::Service
    //    after extraction some strange type error on app.call(req) pops up
    //    it looks like some constraints on associated type not being satisfied
    #[actix_rt::test]
    async fn test_no_jump() {
        let jumps_repo = Arc::new(JumpsRepositoryMock::new(vec![(
            "123",
            "https://github.com",
        )]));
        let stats_repo = Arc::new(StatsRepositoryMock::new());
        let mut app = test::init_service(
            App::new()
                .data::<Arc<dyn JumpsRepository>>(jumps_repo.clone())
                .data::<Arc<dyn StatsRepository>>(stats_repo.clone())
                .service(get_link),
        )
        .await;

        let req = test::TestRequest::with_uri("/another").to_request();
        let resp = app.call(req).await.unwrap();
        assert_eq!(resp.status(), http::StatusCode::NOT_FOUND);
    }

    #[actix_rt::test]
    async fn test_jump() {
        let jumps_repo = Arc::new(JumpsRepositoryMock::new(vec![(
            "123",
            "https://github.com",
        )]));
        let stats_repo = Arc::new(StatsRepositoryMock::new());
        let mut app = test::init_service(
            App::new()
                .data::<Arc<dyn JumpsRepository>>(jumps_repo.clone())
                .data::<Arc<dyn StatsRepository>>(stats_repo.clone())
                .service(get_link),
        )
        .await;
        let req = test::TestRequest::with_uri("/123").to_request();
        let resp = app.call(req).await.unwrap();
        assert_eq!(resp.status(), http::StatusCode::MOVED_PERMANENTLY);
        assert_eq!(
            resp.headers()
                .get(http::header::LOCATION)
                .unwrap()
                .to_str()
                .unwrap(),
            "https://github.com"
        );
    }
}
