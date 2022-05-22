use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use tokio::sync::OnceCell;

use crate::config::Config;

pub struct Db;

impl Db {
    async fn connect() -> DatabaseConnection {
        let app_config = Config::get().await;
        let connect_options = ConnectOptions::new(app_config.db_url.clone());
        return Database::connect(connect_options)
            .await
            .expect("Database connection failed");
    }
    pub async fn get_conn() -> &'static DatabaseConnection {
        static DB: OnceCell<DatabaseConnection> = OnceCell::const_new();
        return DB.get_or_init(Self::connect).await;
    }
}
