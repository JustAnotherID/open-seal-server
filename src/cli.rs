use crate::api::ApiState;
use crate::config::{read_config, Config};
use crate::{
    api::base::{health, root},
    api::download::download,
    api::upload::upload,
    db::conn::establish_conn,
};
use anyhow::Error;
use axum::{
    extract::DefaultBodyLimit,
    routing::{get, put},
    Router,
};
use log::info;
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

pub async fn run() -> Result<(), Error> {
    let conf = read_config()?;
    server_start(conf).await?;
    Ok(())
}

async fn server_start(config: Config) -> Result<(), Error> {
    let db_config = config.database.clone();
    let db = establish_conn(db_config).await?;

    let server_conf = config.server.clone();
    let state = ApiState { db, config };
    let app = Router::new()
        .route("/", get(root))
        .route("/health", get(health))
        .route(
            "/dice/api/log",
            put(upload).layer(DefaultBodyLimit::max(1024 * server_conf.max_log_mb)),
        )
        .route("/dice/api/load_data", get(download))
        .with_state(state)
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive());
    let listener = TcpListener::bind(format!("{}:{}", server_conf.host, server_conf.port)).await?;
    info!(
        "Starting seal story painter server, listening on {}:{}",
        server_conf.host, server_conf.port
    );
    Ok(axum::serve(listener, app).await?)
}
