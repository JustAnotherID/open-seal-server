use crate::api::root::not_found;
use crate::{
    api::{
        root::static_handler,
        story_log::{download::download, health::health, upload::upload},
        ApiState,
    },
    config::{read_config, Config},
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
use tower_http::{cors::CorsLayer, trace::TraceLayer};

pub async fn run() -> Result<(), Error> {
    let conf = read_config()?;
    server_start(conf).await?;
    Ok(())
}

async fn server_start(config: Config) -> Result<(), Error> {
    let server_conf = config.server.clone();
    let db_config = config.database.clone();
    let db = establish_conn(db_config).await?;

    let story_log_config = config.story_log.clone();
    let state = ApiState { db, config };

    let app = Router::new()
        .route("/health", get(health))
        // story log api
        .fallback(static_handler) // fallback to painter page
        .route(
            "/dice/api/log",
            put(upload).layer(DefaultBodyLimit::max(1024 * story_log_config.max_log_mb)),
        )
        .route("/dice/api/load_data", get(download))
        .with_state(state)
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive());
    let listener = TcpListener::bind(format!("{}:{}", server_conf.host, server_conf.port)).await?;
    info!("listening on {}:{}", server_conf.host, server_conf.port);
    Ok(axum::serve(listener, app).await?)
}
