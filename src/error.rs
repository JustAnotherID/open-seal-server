use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub enum ApiError {
    Internal(anyhow::Error),
    Param(anyhow::Error),
    NotFound(anyhow::Error),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            ApiError::Internal(err) => {
                (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response()
            }
            ApiError::Param(err) => (StatusCode::BAD_REQUEST, err.to_string()).into_response(),
            ApiError::NotFound(err) => (StatusCode::NOT_FOUND, err.to_string()).into_response(),
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
