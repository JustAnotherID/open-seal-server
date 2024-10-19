use crate::config::Config;
use axum::{
    body::Body,
    extract::{Path, State},
    http::{header, StatusCode},
    response::IntoResponse,
};
use regex::Regex;
use tokio_util::io::ReaderStream;

pub async fn download(
    Path(extension_id): Path<String>,
    State(config): State<Config>,
) -> impl IntoResponse {
    let extension_id_regex =
        Regex::new(r"^@(?<namespace>.*?)/(?<key>.*?)@(?<version>[a-zA-Z0-9+\-.]+)$").unwrap();
    if extension_id.is_empty() {
        return Err((StatusCode::BAD_REQUEST, "extension id is empty").into_response());
    } else if !extension_id_regex.is_match(&extension_id) {
        return Err((StatusCode::BAD_REQUEST, "invalid extension id").into_response());
    }

    let store_config = config.store;
    let target = std::path::Path::new(&store_config.extension_dir).join("demo");
    let file = match tokio::fs::File::open(target).await {
        Ok(file) => file,
        Err(_) => {
            return Err((
                StatusCode::NOT_FOUND,
                format!("target extension \"{}\" not found", extension_id),
            )
                .into_response())
        }
    };
    let stream = ReaderStream::new(file);
    let body = Body::from_stream(stream);

    let headers = [
        (header::CONTENT_TYPE, "application/octet-stream".to_string()),
        (
            header::CONTENT_DISPOSITION,
            format!("attachment; filename=\"{}\"", extension_id),
        ),
    ];
    Ok((headers, body))
}
