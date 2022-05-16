use sea_orm::{ConnectOptions, Database, DatabaseConnection};

use crate::config::APP_CONFIG;

pub async fn get_pool() -> DatabaseConnection {
    let db_url = APP_CONFIG.db_url.clone();
    let opt = ConnectOptions::new(db_url);
    return Database::connect(opt)
        .await
        .expect("Database connection failed");
}
