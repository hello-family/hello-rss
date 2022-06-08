use axum::{routing::post, Router};

mod user;

fn user() -> Router {
    Router::new().route("/signup", post(user::signup))
    // .route("/activate", get(user::activate))
    // .route("/login", get(user::login))
    // .route("/info", get(user::info))
    // .route("/logout", get(user::logout))
    // .route("/change_password", get(user::change_password))
    // .route("/change_email", get(user::change_email))
    // .route("/change_email_confirm", get(user::change_email_confirm))
    // .route("/reset_password", get(user::reset_password))
    // .route("/reset_password_confirm", get(user::reset_password_confirm))
}

fn rss() -> Router {
    Router::new()
    // .route("/channel_add", post(rss::channel_add))
    // .route("/channel_delete", post(rss::channel_delete))
    // .route("/channel_update", post(rss::channel_update))
    // .route("/channel_list", get(rss::channel_list))
    // .route("/channel_info", get(rss::channel_info))
    // .route("/channel_fetch", get(rss::channel_fetch))
    // .route("/channel_read_all", get(rss::channel_read_all))
    // .route("/item_list", get(rss::item_list))
    // .route("/item_read", get(rss::item_read))
}

 fn favorite() -> Router {
    Router::new()
    // .route("/list", get(favorite::list))
    // .route("/add", post(favorite::add))
    // .route("/delete", post(favorite::delete));
}


pub fn router() -> Router {
    Router::new()
        .nest("/user", user())
        .nest("/rss", rss())
        .nest("/favorite", favorite())
}
