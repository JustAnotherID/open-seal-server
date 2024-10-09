use anyhow::{anyhow, Error};
use base64::{prelude::BASE64_STANDARD, Engine};
use entity::entities::{
    file_info::{ActiveModel, Column, Model},
    prelude::FileInfo,
};
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

pub async fn save_file_info(
    db: &DatabaseConnection,
    name: &str,
    uniform_id: &str,
    content: Vec<u8>,
) -> Result<(String, String), Error> {
    let now = time::OffsetDateTime::now_utc();
    let exists: Option<Model> = FileInfo::find()
        .filter(Column::Name.eq(name))
        .filter(Column::UniformId.eq(uniform_id))
        .one(db)
        .await?;
    if let Some(model) = exists {
        let key = model.key.clone();
        let secret = model.secret.clone();

        let mut update: ActiveModel = model.into();
        update.updated_at = Set(Option::from(now));
        update.content = Set(content);
        update.update(db).await?;
        Ok((key, secret))
    } else {
        let (key, secret) = loop {
            let (key, secret) = random_key_secret();
            let generated = FileInfo::find()
                .filter(Column::Key.eq(&key))
                .count(db)
                .await?
                > 0;
            if !generated {
                break (key, secret);
            }
        };
        let model = ActiveModel {
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

#[derive(serde::Serialize)]
pub struct FileInfoDTO {
    pub name: String,
    pub data: String,
    pub created_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

pub async fn find_file_info(
    db: &DatabaseConnection,
    key: &str,
    secret: &str,
) -> Result<FileInfoDTO, Error> {
    let model = FileInfo::find()
        .filter(Column::Key.eq(key))
        .filter(Column::Secret.eq(secret))
        .one(db)
        .await
        .expect("log data not found");
    if let Some(model) = model {
        let mut encoder = ZlibEncoder::new(Vec::new(), flate2::Compression::default());
        encoder.write_all(&model.content)?;
        let data = encoder.finish()?;
        return Ok(FileInfoDTO {
            name: model.name,
            data: BASE64_STANDARD.encode(data),
            created_at: model.created_at.to_string(),
            updated_at: model.updated_at.map(|v| v.to_string()),
        });
    }
    Err(anyhow!("log data not found"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::conn::set_conn;
    use migration::{Migrator, MigratorTrait};
    use sea_orm::{ConnectOptions, Database};

    #[tokio::test]
    async fn test_random_key_secret() {
        let (key, secret) = random_key_secret();
        assert_eq!(key.len(), KEY_SECRET_LENGTH);
        assert_eq!(secret.len(), KEY_SECRET_LENGTH);
    }

    async fn test_db() -> DatabaseConnection {
        let mut opt = ConnectOptions::new("sqlite::memory:");
        opt = set_conn(opt);
        let db = Database::connect(opt).await.unwrap();
        Migrator::fresh(&db).await.unwrap();
        db
    }

    #[tokio::test]
    async fn test_save_file_info() {
        let db = test_db().await;
        let (key1, secret1) = save_file_info(&db, "test", "QQ:114514", vec![])
            .await
            .unwrap();
        assert_eq!(key1.len(), KEY_SECRET_LENGTH);
        assert_eq!(secret1.len(), KEY_SECRET_LENGTH);

        let (key2, secret2) = save_file_info(&db, "test", "QQ:114514", Vec::from("new content"))
            .await
            .unwrap();
        assert_eq!(key1, key2);
        assert_eq!(secret1, secret2);

        let (key3, secret3) = save_file_info(&db, "another test", "QQ:1919810", vec![])
            .await
            .unwrap();
        assert_ne!(key1, key3);
        assert_ne!(secret1, secret3);
    }
}
