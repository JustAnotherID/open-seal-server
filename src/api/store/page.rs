use crate::{
    api::{store::Extension, Response},
    db::{Page, Paging},
};
use axum::extract::Query;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub(crate) struct QueryParams {
    #[serde(flatten)]
    pub(crate) paging: Paging,

    pub(crate) r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) author: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) sort_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) order: Option<String>,
}

pub async fn page(Query(params): Query<QueryParams>) -> Response<Page<Extension>> {
    todo!()
}
