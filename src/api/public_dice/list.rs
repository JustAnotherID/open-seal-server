use crate::{
    error::ApiError,
    service::{dice_info::DiceInfo, Paging},
};
use axum::{
    extract::{Query, State},
    Json,
};
use sea_orm::{prelude::DateTimeWithTimeZone, DatabaseConnection};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiceListReq {
    #[serde(flatten, default)]
    pub(crate) paging: Paging,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DiceInfoDTO {
    pub(crate) id: String,
    pub(crate) created_at: DateTimeWithTimeZone,
    pub(crate) updated_at: Option<DateTimeWithTimeZone>,
    pub(crate) name: String,
    pub(crate) brief: String,
    pub(crate) note: String,
    pub(crate) avatar: String,
    pub(crate) version: String,
    pub(crate) is_official_version: bool,
    pub(crate) update_tick_count: i64,
    pub(crate) last_tick_time: i64,
    pub(crate) endpoints: Option<Vec<DiceEndpoint>>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DiceEndpoint {
    pub(crate) platform: String,
    pub(crate) uid: String,
    pub(crate) invite_url: String,
    pub(crate) is_online: bool,

    pub(crate) id: String,
    pub(crate) created_at: String,
    pub(crate) updated_at: String,
    pub(crate) last_tick_time: u64,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DiceListResp {
    pub items: Vec<DiceInfoDTO>,
}

pub(crate) async fn list(
    Query(params): Query<DiceListReq>,
    State(db): State<DatabaseConnection>,
) -> Result<Json<DiceListResp>, ApiError> {
    let data = DiceInfo::list(&db, params.paging).await?;
    let result = DiceListResp {
        items: data
            .into_iter()
            .map(|item| DiceInfoDTO {
                id: "-".to_string(),
                created_at: item.created_at,
                updated_at: item.updated_at,
                name: item.name,
                brief: item.brief,
                note: item.note,
                avatar: item.avatar,
                version: item.version,
                is_official_version: true,
                update_tick_count: item.update_tick_count,
                last_tick_time: item.last_tick_time,
                endpoints: None,
            })
            .collect(),
    };
    Ok(Json(result))
}
