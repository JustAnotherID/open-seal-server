use crate::{config::Config, error::ApiError};
use axum::{extract::State, Json};
use serde::Serialize;

const PROTOCOL_VERSIONS: &[&str; 1] = &["1.0.0"];

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BackendInfo {
    pub id: String,
    pub name: String,
    pub protocol_versions: Vec<String>,
    pub announcement: String,
}

impl BackendInfo {
    pub fn new(id: &str, name: &str, announcement: String) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            protocol_versions: PROTOCOL_VERSIONS.map(|s| s.to_string()).to_vec(),
            announcement,
        }
    }
}

pub async fn info(State(config): State<Config>) -> Result<Json<BackendInfo>, ApiError> {
    let store_config = config.store;
    Ok(Json(BackendInfo::new(
        &store_config.id,
        &store_config.name,
        store_config.announcement,
    )))
}
