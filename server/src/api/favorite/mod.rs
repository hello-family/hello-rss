use axum::{routing::post, Router};

pub fn router() -> Router {
    Router::new()
    // .route("/list", get(favorite::list))
    // .route("/add", post(favorite::add))
    // .route("/delete", post(favorite::delete));
}
