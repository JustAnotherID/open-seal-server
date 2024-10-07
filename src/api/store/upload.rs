use crate::{
    config::{Config, UploadFormElem},
    error::ApiError,
};
use axum::{extract::State, Json};
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UploadInfo {
    pub upload_notice: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upload_form: Option<Vec<UploadFormElem>>,
}

impl UploadInfo {
    pub fn new(upload_notice: String, upload_form: Option<Vec<UploadFormElem>>) -> Self {
        Self {
            upload_notice,
            upload_form,
        }
    }
}

pub async fn upload_info(State(config): State<Config>) -> Result<Json<UploadInfo>, ApiError> {
    let store_config = config.store;
    Ok(Json(UploadInfo::new(
        store_config.upload_notice.unwrap_or("".to_string()),
        store_config.upload_form,
    )))
}

pub async fn upload() -> &'static str {
    todo!()
}
