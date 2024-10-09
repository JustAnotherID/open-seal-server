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
