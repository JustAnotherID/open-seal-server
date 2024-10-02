use crate::db::conn::establish_conn;
use crate::{
    api::base::{health, root},
    api::download::download,
    api::upload::upload,
};
use anyhow::Error;
use axum::{
    extract::DefaultBodyLimit,
    routing::{get, put},
    Router,
};
use clap::Parser;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing::info;

mod api;
mod db;
mod error;

#[derive(Parser)]
#[command(about, author, version, long_about = None)]
pub struct Cli {
    #[arg(long, default_value = "localhost")]
    pub host: String,

    #[arg(short, long, default_value_t = 3212)]
    pub port: u16,

    #[arg(short, long, default_value_t = 64)]
    pub max_log_mb: usize,
}

pub async fn run(cli: Cli) -> Result<(), Error> {
    tracing_subscriber::fmt::init();

    let db = establish_conn().await?;

    let app = Router::new()
        .route("/", get(root))
        .route("/health", get(health))
        .route(
            "/dice/api/log",
            put(upload).layer(DefaultBodyLimit::max(1024 * cli.max_log_mb)),
        )
        .route("/dice/api/load_data", get(download))
        .with_state(db)
        .layer(TraceLayer::new_for_http());
    let listener = TcpListener::bind(format!("{}:{}", cli.host, cli.port)).await?;
    info!(
        "starting seal-story-painter server, listening on {}:{}",
        cli.host, cli.port
    );
    Ok(axum::serve(listener, app).await?)
}
