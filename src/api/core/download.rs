use crate::config::Config;
use axum::{
    extract::{Path, State},
    http::{
        header::{self},
        StatusCode,
    },
    response::IntoResponse,
};
use axum_extra::{headers::Range, TypedHeader};
use axum_range::{KnownSize, Ranged};
use header::{HeaderValue, CONTENT_DISPOSITION, CONTENT_TYPE};

pub(crate) async fn download(
    Path(target_file): Path<String>,
    State(config): State<Config>,
    range: Option<TypedHeader<Range>>,
) -> impl IntoResponse {
    let path = std::path::Path::new(&target_file);
    if path.parent() != Some(std::path::Path::new("")) || path.is_absolute() {
        return Err((StatusCode::BAD_REQUEST, "invalid file".to_string()).into_response());
    }

    let core_config = config.core;
    let target = std::path::Path::new(&core_config.file_dir).join(&target_file);
    let filename = path.file_name().and_then(|n| n.to_str()).unwrap_or("file");

    let mut headers = header::HeaderMap::new();
    headers.insert(
        CONTENT_TYPE,
        HeaderValue::from_static("application/octet-stream"),
    );
    headers.insert(
        CONTENT_DISPOSITION,
        HeaderValue::try_from(format!("attachment; filename={}", filename))
            .unwrap_or_else(|_| HeaderValue::from_static("attachment; filename=unknown")),
    );

    let file = tokio::fs::File::open(target).await.unwrap();
    let body = match KnownSize::file(file).await {
        Ok(body) => body,
        Err(_) => return Err((StatusCode::INTERNAL_SERVER_ERROR,).into_response()),
    };

    let ranged = Ranged::new(range.map(|h| h.0), body);
    Ok((headers, ranged))
}
