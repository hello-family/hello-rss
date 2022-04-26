use axum::{
    extract::{Extension, Form, Path, Query},
    http::StatusCode,
    routing::{get, get_service, post},
    Router, Server,
};
use migration::{Migrator, MigratorTrait};
use sea_orm::{prelude::*, ConnectOptions, Database, QueryOrder, Set};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use std::time::Duration;
use std::{env, net::SocketAddr};
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env::set_var("RUST_LOG", "debug");
    tracing_subscriber::fmt::init();

    dotenv::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file");
    let server_url = format!("{}:{}", host, port);

    let mut opt = ConnectOptions::new(db_url);
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true);
    let conn = Database::connect(opt)
        .await
        .expect("Database connection failed");

    Migrator::up(&conn, None).await.unwrap();

    let app = Router::new().layer(
        ServiceBuilder::new()
            .layer(TraceLayer::new_for_http())
            .layer(Extension(conn)),
    );

    let addr = SocketAddr::from_str(&server_url).unwrap();
    Server::bind(&addr).serve(app.into_make_service()).await?;

    Ok(())
}
