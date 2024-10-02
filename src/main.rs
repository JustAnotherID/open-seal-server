use anyhow::Error;
use clap::Parser;
use seal_story_painter_backend::{run, Cli};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let cli = Cli::parse();

    run(cli).await
}
