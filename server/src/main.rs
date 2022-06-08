#[macro_use]
extern crate serde;

mod api;
mod app;
mod config;
mod db;
mod dto;
mod error;
mod extractor;
mod service;
mod utils;

use entity::async_graphql;

use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
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
        .serve(app().await.into_make_service())
        .await?;

    Ok(())
}
