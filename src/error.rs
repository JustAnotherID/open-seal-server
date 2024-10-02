use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

pub enum ApiError {
    Internal(anyhow::Error),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            ApiError::Internal(err) => {
                (StatusCode::INTERNAL_SERVER_ERROR, format!("{}", err)).into_response()
            }
        }
    }
}

impl<E> From<E> for ApiError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self::Internal(err.into())
    }
}
