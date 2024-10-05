use anyhow::Error;
use seal_story_painter_backend::cli;
use tracing_subscriber::{filter::LevelFilter, fmt, fmt::time::OffsetTime, prelude::*, EnvFilter};

fn main() -> Result<(), Error> {
    init_logger();
    run()
}

#[tokio::main]
async fn run() -> Result<(), Error> {
    cli::run().await
}

fn init_logger() {
    tracing_subscriber::registry()
        .with(
            fmt::layer()
                .with_target(false)
                .with_timer(OffsetTime::local_rfc_3339().expect("could not get local offset!")),
        )
        .with(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env_lossy(),
        )
        .init();
}
