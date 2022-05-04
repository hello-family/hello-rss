use std::time::Duration;

use axum::{error_handling::HandleErrorLayer, BoxError, Extension, Router};
use http::{Request, StatusCode};
use migration::{Migrator, MigratorTrait};
use tower::{timeout::TimeoutLayer, ServiceBuilder};
use tower_http::{
    cors::{Any, CorsLayer},
    request_id::{MakeRequestId, RequestId},
    trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer},
    ServiceBuilderExt,
};
use uuid::Uuid;

use crate::{config::Config, db::db, router::router};

#[derive(Clone, Copy)]
struct MakeRequestUuid;

impl MakeRequestId for MakeRequestUuid {
    fn make_request_id<B>(&mut self, request: &Request<B>) -> Option<RequestId> {
        let request_id = Uuid::new_v4().to_string().parse().unwrap();
        Some(RequestId::new(request_id))
    }
}

pub async fn app(config: Config) -> Router {
    let db = db(config.db_url).await;
    Migrator::up(&db, None).await.unwrap();

    router().layer(
        ServiceBuilder::new()
            .set_x_request_id(MakeRequestUuid)
            .propagate_x_request_id()
            // this middleware goes above `TimeoutLayer` because it will receive
            // errors returned by `TimeoutLayer`
            .layer(HandleErrorLayer::new(|_: BoxError| async {
                StatusCode::REQUEST_TIMEOUT
            }))
            .layer(
                TraceLayer::new_for_http()
                    .make_span_with(DefaultMakeSpan::new().include_headers(true))
                    .on_response(DefaultOnResponse::new().include_headers(true)),
            )
            .layer(CorsLayer::new().allow_origin(Any))
            .layer(TimeoutLayer::new(Duration::from_secs(10)))
            .layer(Extension(db)),
    )
}
