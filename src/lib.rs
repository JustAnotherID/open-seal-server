use crate::{
    api::base::{health, root},
    api::download::download,
    api::upload::upload,
};
use anyhow::Error;
use axum::{
    routing::{get, put},
    Router,
};
use clap::Parser;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;

mod api;
mod error;

#[derive(Parser)]
#[command(about, author, version, long_about = None)]
pub struct Cli {
    #[arg(long, default_value = "localhost")]
    pub host: String,

    #[arg(short, long, default_value_t = 3212)]
    pub port: u16,
}

pub async fn run(cli: Cli) -> Result<(), Error> {
    let app = Router::new()
        .route("/", get(root))
        .route("/health", get(health))
        .route("/dice/api/log", put(upload))
        .route("/dice/api/load_data", get(download))
        .layer(TraceLayer::new_for_http());
    let listener = TcpListener::bind(format!("{}:{}", cli.host, cli.port)).await?;
    Ok(axum::serve(listener, app).await?)
}
