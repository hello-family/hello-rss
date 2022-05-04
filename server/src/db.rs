use sea_orm::{ConnectOptions, Database, DatabaseConnection};

pub async fn db(db_url: String) -> DatabaseConnection {
    let opt = ConnectOptions::new(db_url);
    return Database::connect(opt)
        .await
        .expect("Database connection failed");
}
