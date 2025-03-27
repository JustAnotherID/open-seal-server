use crate::{
    api::public_dice::KeyPayload, config::Config, error::ApiError, service::dice_info::DiceInfo,
};
use axum::{extract::State, Json};
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TickUpdateReq {
    #[validate(length(min = 1))]
    pub id: String,
    #[validate(length(min = 1))]
    pub key: String,
    #[validate(nested)]
    pub endpoints: Option<Vec<TickEndpoint>>,
}

#[derive(Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TickEndpoint {
    #[validate(length(min = 1))]
    pub uid: String,
    pub is_online: bool,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TickUpdateResp {}

pub(crate) async fn tick_update(
    State(db): State<DatabaseConnection>,
    State(_config): State<Config>,
    Json(tick_info): Json<TickUpdateReq>,
) -> Result<Json<TickUpdateResp>, ApiError> {
    tick_info.validate()?;
    let _payload = KeyPayload::from_key(tick_info.key)?;

    if let Some(endpoints) = tick_info.endpoints {
        DiceInfo::tick_update(
            &db,
            &tick_info.id,
            endpoints
                .into_iter()
                .filter(|x| x.is_online)
                .map(|x| x.uid)
                .collect(),
        )
        .await?;
    } else {
        DiceInfo::tick_update(&db, &tick_info.id, vec![]).await?;
    };

    Ok(Json(TickUpdateResp {}))
}
