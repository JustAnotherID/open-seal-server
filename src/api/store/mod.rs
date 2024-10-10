use entity::entities::extension;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub(crate) mod download;
pub(crate) mod info;
pub(crate) mod page;
pub(crate) mod rating;
pub(crate) mod recommend;
pub(crate) mod upload;

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub(crate) enum ExtType {
    Plugin,
    Deck,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Extension {
    pub(crate) id: String,
    pub(crate) key: String,
    pub(crate) namespace: String,
    pub(crate) version: String,

    pub(crate) r#type: ExtType,
    pub(crate) ext: String,
    pub(crate) name: String,
    pub(crate) authors: Vec<String>,
    pub(crate) desc: String,
    pub(crate) license: String,
    pub(crate) release_time: u64,
    pub(crate) update_time: u64,
    pub(crate) download_num: u64,
    pub(crate) download_url: String,
    pub(crate) hash: HashMap<String, String>,

    pub(crate) tags: Option<Vec<String>>,
    pub(crate) rate: Option<f64>,
    pub(crate) extra: Option<HashMap<String, String>>,
    pub(crate) home_page: Option<String>,
    pub(crate) seal_version: Option<String>,
    pub(crate) dependencies: Option<HashMap<String, String>>,
}

impl From<extension::Model> for Extension {
    fn from(model: extension::Model) -> Self {
        let id = format!("@{}/{}@{}", model.namespace, model.key, model.version);
        let tp = match model.r#type.as_str() {
            "deck" => ExtType::Deck,
            _ => ExtType::Plugin,
        };
        let authors = model
            .authors
            .as_array()
            .unwrap()
            .to_owned()
            .iter()
            .map(|v| v.to_string())
            .collect();
        let tags = model.tags.map(|v| {
            v.as_array()
                .unwrap()
                .to_owned()
                .iter()
                .map(|v| v.to_string())
                .collect()
        });
        let key = model.key;
        let namespace = model.namespace;
        let version = model.version;
        let ext = model.ext;
        let name = model.name;
        let desc = model.desc;
        let license = model.license;
        let release_time = model.release_time as u64;
        let update_time = model.update_time as u64;
        let download_num = model.download_num as u64;
        let download_url = "".to_string();
        let hash = HashMap::new();
        Self {
            id,
            key,
            namespace,
            version,
            r#type: tp,
            ext,
            name,
            authors,
            desc,
            license,
            release_time,
            update_time,
            download_num,
            download_url,
            hash,
            tags,
            rate: None,
            extra: None,
            home_page: None,
            seal_version: None,
            dependencies: None,
        }
    }
}
