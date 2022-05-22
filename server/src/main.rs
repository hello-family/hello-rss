#[macro_use]
extern crate serde;

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
use config::Config;
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
    let app_config = Config::get().await;
    let addr = SocketAddr::from_str(&app_config.server_url).unwrap();
    Server::bind(&addr)
        .serve(app().await.into_make_service())
        .await?;

    Ok(())
}
