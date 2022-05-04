mod api;
mod app;
mod config;
mod db;
mod router;
mod validator;

use crate::config::config;
use app::app;
use axum::Server;
#[cfg(debug_assertions)]
use dotenv::dotenv;
use std::net::SocketAddr;
use std::str::FromStr;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    #[cfg(debug_assertions)]
    dotenv().ok();
    tracing_subscriber::fmt()
        .pretty()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let config = config();

    let addr = SocketAddr::from_str(&config.server_url).unwrap();
    Server::bind(&addr)
        .serve(app(config).await.into_make_service())
        .await?;

    Ok(())
}
