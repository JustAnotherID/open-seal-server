use crate::{
    api::public_dice::KeyPayload, config::Config, error::ApiError, service::dice_info::DiceInfo,
};
use axum::{
    extract::{Query, State},
    Json,
};
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize)]
pub struct RegisterMark {
    pub update: Option<u8>,
}

#[derive(Default, Serialize, Deserialize, Validate)]
#[serde(default, rename_all = "camelCase")]
pub struct RegisterReq {
    #[serde(rename = "ID")]
    pub id: Option<String>,
    pub name: String,
    pub brief: String,
    pub note: String,
    #[validate(url)]
    pub avatar: Option<String>,
    pub key: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterInfo {
    pub id: Option<String>,
    pub name: String,
    pub brief: String,
    pub note: String,
    pub avatar: String,
    pub version: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum RegisterResp {
    Register { item: RegisterInfo },
    Update { update: RegisterInfo },
}

pub(crate) async fn register_or_update(
    State(db): State<DatabaseConnection>,
    State(_config): State<Config>,
    Query(mark): Query<RegisterMark>,
    Json(register_info): Json<RegisterReq>,
) -> Result<Json<RegisterResp>, ApiError> {
    register_info.validate()?;
    let payload = KeyPayload::from_key(register_info.key)?;

    let record = DiceInfo::register_or_update_dice(
        &db,
        &register_info.id,
        &register_info.name,
        &register_info.brief,
        &register_info.note,
        &register_info.avatar.unwrap_or(String::new()),
        &payload.version,
    )
    .await?;

    let result = match mark.update {
        Some(1) => RegisterResp::Update {
            update: RegisterInfo {
                id: Some(record.openid),
                name: record.name,
                brief: record.brief,
                note: record.note,
                avatar: record.avatar,
                version: record.version,
            },
        },
        _ => RegisterResp::Register {
            item: RegisterInfo {
                id: Some(record.openid),
                name: record.name,
                brief: record.brief,
                note: record.note,
                avatar: record.avatar,
                version: record.version,
            },
        },
    };
    Ok(Json(result))
}
