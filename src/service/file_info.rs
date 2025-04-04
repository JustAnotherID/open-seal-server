use anyhow::{anyhow, Error};
use base64::{prelude::BASE64_STANDARD, Engine};
use chrono::Utc;
use entity::entities::{file_info, prelude};
use flate2::write::ZlibEncoder;
use nid::{alphabet::Base36LowercaseAlphabet, Nanoid};
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait,
    PaginatorTrait, QueryFilter,
};
use std::io::Write;

const KEY_SECRET_LENGTH: usize = 6;

fn random_key_secret() -> (String, String) {
    let key: Nanoid<KEY_SECRET_LENGTH, Base36LowercaseAlphabet> = Nanoid::new();
    let secret: Nanoid<KEY_SECRET_LENGTH, Base36LowercaseAlphabet> = Nanoid::new();
    (key.to_string(), secret.to_string())
}

pub(crate) struct FileInfo {}

#[derive(serde::Serialize)]
pub(crate) struct FileInfoDTO {
    pub name: String,
    pub data: String,
    pub created_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}
impl FileInfo {
    pub(crate) async fn save_file_info(
        db: &DatabaseConnection,
        name: &str,
        uniform_id: &str,
        content: Vec<u8>,
    ) -> Result<(String, String), Error> {
        let now = Utc::now().fixed_offset();
        let exists: Option<file_info::Model> = prelude::FileInfo::find()
            .filter(file_info::Column::Name.eq(name))
            .filter(file_info::Column::UniformId.eq(uniform_id))
            .one(db)
            .await?;
        if let Some(model) = exists {
            let key = model.key.clone();
            let secret = model.secret.clone();

            let mut update: file_info::ActiveModel = model.into();
            update.updated_at = Set(Option::from(now));
            update.content = Set(content);
            update.update(db).await?;
            Ok((key, secret))
        } else {
            let (key, secret) = loop {
                let (key, secret) = random_key_secret();
                let generated = prelude::FileInfo::find()
                    .filter(file_info::Column::Key.eq(&key))
                    .count(db)
                    .await?
                    > 0;
                if !generated {
                    break (key, secret);
                }
            };
            let model = file_info::ActiveModel {
                name: Set(name.to_string()),
                uniform_id: Set(uniform_id.to_string()),
                key: Set(key.clone()),
                secret: Set(secret.clone()),
                created_at: Set(now),
                content: Set(content),
                ..Default::default()
            };
            model.insert(db).await?;
            Ok((key, secret))
        }
    }

    pub async fn find_file_info(
        db: &DatabaseConnection,
        key: &str,
        secret: &str,
    ) -> Result<FileInfoDTO, Error> {
        let model = prelude::FileInfo::find()
            .filter(file_info::Column::Key.eq(key))
            .filter(file_info::Column::Secret.eq(secret))
            .one(db)
            .await?
            .ok_or(anyhow!("log data not found"))?;

        let mut encoder = ZlibEncoder::new(Vec::new(), flate2::Compression::default());
        encoder.write_all(&model.content)?;
        let data = encoder.finish()?;
        Ok(FileInfoDTO {
            name: model.name,
            data: BASE64_STANDARD.encode(data),
            created_at: model.created_at.to_string(),
            updated_at: model.updated_at.map(|v| v.to_string()),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::service::conn::build_test_db;

    use super::*;

    #[tokio::test]
    async fn test_random_key_secret() {
        let (key, secret) = random_key_secret();
        assert_eq!(key.len(), KEY_SECRET_LENGTH);
        assert_eq!(secret.len(), KEY_SECRET_LENGTH);
    }

    #[tokio::test]
    async fn test_save_file_info() {
        let db = build_test_db().await;
        let (key1, secret1) = FileInfo::save_file_info(&db, "test", "QQ:114514", vec![])
            .await
            .unwrap();
        assert_eq!(key1.len(), KEY_SECRET_LENGTH);
        assert_eq!(secret1.len(), KEY_SECRET_LENGTH);

        let (key2, secret2) =
            FileInfo::save_file_info(&db, "test", "QQ:114514", Vec::from("new content"))
                .await
                .unwrap();
        assert_eq!(key1, key2);
        assert_eq!(secret1, secret2);

        let (key3, secret3) = FileInfo::save_file_info(&db, "another test", "QQ:1919810", vec![])
            .await
            .unwrap();
        assert_ne!(key1, key3);
        assert_ne!(secret1, secret3);
    }
}
