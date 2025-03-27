use crate::{config::Config, error::ApiError, service::file_info::FileInfo};
use anyhow::Error;
use axum::{
    extract::{Multipart, State},
    Json,
};
use flate2::bufread::ZlibDecoder;
use sea_orm::DatabaseConnection;
use std::{collections::HashMap, io::Read, str::from_utf8, string::String};

pub async fn upload(
    State(db): State<DatabaseConnection>,
    State(config): State<Config>,
    mut multipart: Multipart,
) -> Result<Json<HashMap<String, String>>, ApiError> {
    let mut name = "".to_string();
    let mut uniform_id = "".to_string();
    let mut _client = "".to_string();
    let mut _version = "".to_string();
    let mut url = "".to_string();
    while let Some(field) = multipart.next_field().await? {
        match field.name() {
            Some("name") => {
                let b = field.bytes().await?;
                name = from_utf8(b.to_vec().as_slice())?.to_string();
            }
            Some("uniform_id") => {
                let b = field.bytes().await?;
                uniform_id = from_utf8(b.to_vec().as_slice())?.to_string();
            }
            Some("client") => {
                let b = field.bytes().await?;
                _client = from_utf8(b.to_vec().as_slice())?.to_string();
            }
            Some("version") => {
                let b = field.bytes().await?;
                _version = from_utf8(b.to_vec().as_slice())?.to_string();
            }
            Some("file") => {
                let _ = field.file_name().unwrap();
                let data = field.bytes().await?;

                url = write_log_file(&db, &name, &uniform_id, &data)
                    .await
                    .map_or(String::new(), |(key, secret)| {
                        format!("{}?key={}#{}", config.server.domain, key, secret)
                    });
            }
            _ => break,
        }
    }
    let res: HashMap<String, String> = [("url".to_string(), url)].into_iter().collect();
    Ok(Json(res))
}

async fn write_log_file(
    db: &DatabaseConnection,
    name: &str,
    uniform_id: &str,
    compressed_data: &[u8],
) -> Result<(String, String), Error> {
    let mut zlib = ZlibDecoder::new(compressed_data);
    let mut buf = Vec::new();
    zlib.read_to_end(&mut buf).expect("zlib decode failed");
    let (key, secret) = FileInfo::save_file_info(db, name, uniform_id, buf)
        .await
        .expect("save file info failed");

    Ok((key, secret))
}
