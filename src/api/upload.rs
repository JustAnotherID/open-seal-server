use crate::error::ApiError;
use axum::extract::Multipart;
use axum::Json;
use std::collections::HashMap;
use std::str::from_utf8;

pub async fn upload(mut multipart: Multipart) -> Result<Json<HashMap<String, String>>, ApiError> {
    let mut _name: String = "".to_string();
    let mut _uniform_id = "".to_string();
    let mut url: String = "".to_string();
    while let Ok(field) = multipart.next_field().await {
        if let Some(field) = field {
            match field.name() {
                Some("name") => {
                    let b = field.bytes().await?;
                    _name = from_utf8(b.to_vec().as_slice())?.to_string();
                }
                Some("uniform_id") => {
                    let b = field.bytes().await?;
                    _uniform_id = from_utf8(b.to_vec().as_slice())?.to_string();
                }
                // Some("client") => {}
                // Some("version") => {}
                Some("file") => {
                    let _filename = field.file_name().unwrap();
                    let _data = field.bytes().await?;
                    // tokio::fs::write(&filename, &data).await?;
                    url = "http://localhost:3000?key=1024#114514".to_string();
                }
                None | _ => {}
            }
        }
    }
    let res: HashMap<String, String> = [("url".to_string(), url)].iter().cloned().collect();
    Ok(Json(res))
}
