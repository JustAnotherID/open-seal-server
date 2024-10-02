use anyhow::Error;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::time::Duration;
use tracing::log;

pub async fn establish_conn() -> Result<DatabaseConnection, Error> {
    let mut opt = ConnectOptions::new("postgres://postgres:postgres@localhost:5432/postgres");
    opt = set_conn(opt);
    let db = Database::connect(opt)
        .await
        .expect("can't connect to database");
    Ok(db)
}

pub fn set_conn(mut opt: ConnectOptions) -> ConnectOptions {
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Debug)
        .set_schema_search_path("public");
    opt
}
