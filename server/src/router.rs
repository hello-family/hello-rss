use axum::Router;

use crate::api::*;

pub fn router() -> Router {
    Router::new()
        .nest("/user", user())
        .nest("/rss", rss())
        .nest("/favorite", favorite())
}
