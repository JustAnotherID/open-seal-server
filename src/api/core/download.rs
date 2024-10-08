use crate::config::Config;
use axum::{
    body::Body,
    extract::{Path, State},
    http::{header, StatusCode},
    response::IntoResponse,
};
use tokio_util::io::ReaderStream;

pub(crate) async fn download(
    Path(target_file): Path<String>,
    State(config): State<Config>,
) -> impl IntoResponse {
    if target_file.is_empty() {
        return Err((StatusCode::BAD_REQUEST, "target file is empty").into_response());
    } else if target_file.contains("/") {
        return Err((StatusCode::BAD_REQUEST, "illegal target file").into_response());
    }

    let core_config = config.core;
    let target = std::path::Path::new(&core_config.file_dir).join(&target_file);
    let file = match tokio::fs::File::open(target).await {
        Ok(file) => file,
        Err(_) => {
            return Err((
                StatusCode::NOT_FOUND,
                format!("target file \"{}\" not found", target_file),
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
            format!("attachment; filename=\"{}\"", target_file),
        ),
    ];
    Ok((headers, body))
}
