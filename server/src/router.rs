use axum::Router;

use crate::api::*;

pub fn router() -> Router {
    Router::new()
        .nest("/user", user::router())
        .nest("/rss", rss::router())
        .nest("/favorite", favorite::router())
}
