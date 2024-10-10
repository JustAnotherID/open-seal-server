use serde::{Deserialize, Serialize};

pub(crate) mod conn;
pub(crate) mod extension;
pub(crate) mod file_info;

fn page_num_default() -> u64 {
    1
}

fn page_size_default() -> u64 {
    20
}

#[derive(Serialize, Deserialize, Clone, Copy, Default)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Paging {
    #[serde(default = "page_num_default")]
    page_num: u64, // from one
    #[serde(default = "page_size_default")]
    page_size: u64,
}

impl Paging {
    #[allow(dead_code)]
    pub(crate) fn new(page_num: u64, page_size: u64) -> Self {
        let page_num = if page_num > 0 {
            page_num
        } else {
            page_num_default()
        };
        let page_size = if page_size > 0 {
            page_size
        } else {
            page_size_default()
        };
        Self {
            page_num,
            page_size,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Page<T> {
    page_num: u64,
    page_size: u64,
    data: Vec<T>,
    next: bool,
}

impl<T> Page<T> {
    pub(crate) fn new(page_num: u64, page_size: u64, data: Vec<T>, next: bool) -> Self {
        Self {
            page_num,
            page_size,
            data,
            next,
        }
    }

    pub(crate) fn map<U>(self, f: impl FnMut(&T) -> U + Sized) -> Page<U> {
        Page::new(
            self.page_num,
            self.page_size,
            self.data.iter().map(f).collect(),
            self.next,
        )
    }
}
