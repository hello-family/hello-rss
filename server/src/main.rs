#[macro_use]
extern crate serde;
#[macro_use]
extern crate lazy_static;

mod api;
mod app;
mod config;
mod db;
mod dto;
mod error;
mod router;
mod service;
mod utils;

use app::app;
use axum::Server;
use config::APP_CONFIG;
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

    let addr = SocketAddr::from_str(&APP_CONFIG.server_url).unwrap();
    Server::bind(&addr)
        .serve(app(&APP_CONFIG).await.into_make_service())
        .await?;

    Ok(())
}
