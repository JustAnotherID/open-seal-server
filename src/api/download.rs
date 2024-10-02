use crate::error::ApiError;
use axum::extract::Query;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct DownloadParams {
    key: String,
    password: String,
}

pub async fn download(Query(_params): Query<DownloadParams>) -> Result<(), ApiError> {
    todo!()
}
