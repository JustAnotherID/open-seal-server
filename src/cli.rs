use crate::{
    api::{
        core,
        health::health,
        root::{not_found, static_handler},
        store, story_log, ApiState,
    },
    config::{read_config, Config},
    db::conn::establish_conn,
};
use anyhow::Error;
use axum::{
    extract::DefaultBodyLimit,
    routing::{get, post, put},
    Router,
};
use log::info;
use tokio::net::TcpListener;
use tower_http::{compression::CompressionLayer, cors::CorsLayer, trace::TraceLayer};

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

    let store_router = Router::new()
        .route("/info", get(store::info::info))
        .route("/recommend", get(store::recommend::recommend))
        .route("/page", get(store::page::page))
        .route("/download/:key", get(store::download::download))
        .route("/upload/info", get(store::upload::upload_info))
        .route("/upload", post(store::upload::upload))
        .route("/rating", put(store::rating::rating))
        .fallback(not_found);
    let app = Router::new()
        .route("/health", get(health))
        // core api
        .route("/dice/api/version", get(core::version::version))
        .route(
            "/dice/api/core/download/:target_file",
            get(core::download::download),
        )
        // story log api
        .fallback(static_handler) // fallback to painter page
        .route(
            "/dice/api/log",
            put(story_log::upload::upload)
                .layer(DefaultBodyLimit::max(1024 * story_log_config.max_log_mb)),
        )
        .route("/dice/api/load_data", get(story_log::download::download))
        // store api
        .nest("/dice/api/store", store_router)
        .with_state(state)
        .layer(TraceLayer::new_for_http())
        .layer(
            CompressionLayer::new()
                .br(true)
                .deflate(true)
                .gzip(true)
                .zstd(true),
        )
        .layer(CorsLayer::permissive());
    let listener = TcpListener::bind(format!("{}:{}", server_conf.host, server_conf.port)).await?;
    info!("listening on {}:{}", server_conf.host, server_conf.port);
    Ok(axum::serve(listener, app).await?)
}
