use crate::config::Config;
use crate::error::ApiError;
use axum::extract::State;
use axum::response::Html;

pub(crate) async fn news(State(config): State<Config>) -> Result<Html<String>, ApiError> {
    let core_config = config.core;
    Ok(Html(core_config.news_html))
}
