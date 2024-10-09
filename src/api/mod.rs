use crate::config::Config;
use axum::extract::FromRef;
use axum::response::IntoResponse;
use axum::Json;
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};
use serde_json::json;

pub(crate) mod core;
pub(crate) mod health;
pub(crate) mod root;
pub(crate) mod store;
pub(crate) mod story_log;

#[derive(Clone)]
pub(crate) struct ApiState {
    pub(crate) db: DatabaseConnection,
    pub(crate) config: Config,
}

impl FromRef<ApiState> for DatabaseConnection {
    fn from_ref(app_state: &ApiState) -> Self {
        app_state.db.clone()
    }
}

impl FromRef<ApiState> for Config {
    fn from_ref(api_state: &ApiState) -> Self {
        api_state.config.clone()
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub(crate) enum Response<T> {
    Ok { data: T },
    Err { err: String },
}

impl<T: Serialize> IntoResponse for Response<T> {
    fn into_response(self) -> axum::response::Response {
        match self {
            Response::Ok { data } => Json(json!({
                "result": true,
                "data": data,
            }))
            .into_response(),
            Response::Err { err } => Json(json!({
                "result": false,
                "err": err,
            }))
            .into_response(),
        }
    }
}
