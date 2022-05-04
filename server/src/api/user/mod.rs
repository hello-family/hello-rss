use axum::{routing::post, Router};

mod signup;

pub fn router() -> Router {
    Router::new().route("/signup", post(signup::handle))
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
