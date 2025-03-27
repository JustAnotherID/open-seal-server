use crate::config::DbConfig;
use anyhow::Error;
use migration::{Migrator, MigratorTrait};
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::time::Duration;
use tracing::log;

pub(crate) async fn establish_conn(conf: DbConfig) -> Result<DatabaseConnection, Error> {
    let mut opt = match conf {
        DbConfig::Sqlite { path } => ConnectOptions::new(format!("sqlite://{}?mode=rwc", path)),
        DbConfig::Postgres { url } => ConnectOptions::new(url),
    };
    opt = set_conn(opt);
    let db = Database::connect(opt)
        .await
        .expect("can't connect to database");
    Migrator::up(&db, None)
        .await
        .expect("database migration failed");
    Ok(db)
}

pub(crate) fn set_conn(mut opt: ConnectOptions) -> ConnectOptions {
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

#[cfg(test)]
pub(crate) async fn build_test_db() -> DatabaseConnection {
    let mut opt = ConnectOptions::new("sqlite::memory:");
    opt = set_conn(opt);
    let db = Database::connect(opt).await.unwrap();
    Migrator::fresh(&db).await.unwrap();
    db
}
