use axum::{extract::State, Json};
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};

use crate::{
    api::public_dice::KeyPayload,
    config::Config,
    error::ApiError,
    service::dice_endpoint::{DiceEndpoint, EndpointInfoDTO},
};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct EndpointUpdateReq {
    pub dice_id: String,
    pub key: String,
    pub endpoints: Option<Vec<EndpointInfoDTO>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct EndpointUpdateResp {}

pub(crate) async fn endpoint_update(
    State(db): State<DatabaseConnection>,
    State(_config): State<Config>,
    Json(update_info): Json<EndpointUpdateReq>,
) -> Result<Json<EndpointUpdateResp>, ApiError> {
    let _payload = KeyPayload::from_key(update_info.key)?;

    if let Some(endpoints) = update_info.endpoints {
        DiceEndpoint::update_dice_endpoint_info(&db, &update_info.dice_id, endpoints).await?;
    }
    Ok(Json(EndpointUpdateResp {}))
}
