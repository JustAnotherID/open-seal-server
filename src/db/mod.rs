use serde::{Deserialize, Serialize};

pub(crate) mod conn;
pub(crate) mod extension;
pub(crate) mod file_info;

#[derive(Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Paging {
    page_num: i32,
    page_size: i32,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Page<T> {
    page_num: i32,
    page_size: i32,
    data: Vec<T>,
    next: bool,
}
