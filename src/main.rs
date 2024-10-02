use anyhow::Error;
use clap::Parser;
use seal_story_painter_backend::{run, Cli};
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();
    info!(
        "starting seal-story-painter server, listening on {}:{}",
        cli.host, cli.port
    );

    run(cli).await
}
