use crate::config::Config;
use axum::extract::FromRef;
use sea_orm::DatabaseConnection;

pub(crate) mod root;
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
