use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use tokio::sync::OnceCell;

use crate::config::APP_CONFIG;

pub static DB: OnceCell<DatabaseConnection> = OnceCell::const_new();

pub async fn init() -> DatabaseConnection {
    let connect_options = ConnectOptions::new(APP_CONFIG.db_url.clone());
    Database::connect(connect_options).await.unwrap()
}
