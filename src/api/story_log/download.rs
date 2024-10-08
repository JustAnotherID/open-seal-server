use crate::{
    db::file_info::{find_file_info, FileInfoDTO},
    error::ApiError,
};
use axum::{
    extract::{Query, State},
    Json,
};
use sea_orm::DatabaseConnection;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct DownloadParams {
    key: String,
    password: String,
}

pub async fn download(
    Query(params): Query<DownloadParams>,
    State(db): State<DatabaseConnection>,
) -> Result<Json<FileInfoDTO>, ApiError> {
    if params.key.is_empty() || params.password.is_empty() {
        return Err(ApiError::Param(anyhow::anyhow!("key or password is empty")));
    }
    match find_file_info(&db, &params.key, &params.password).await {
        Ok(info) => Ok(Json(info)),
        Err(err) => Err(ApiError::NotFound(err)),
    }
}
