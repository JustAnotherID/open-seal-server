use crate::{
    api::{store, ApiState, Response},
    service::{
        extension::{Extension, SortBy},
        Page, Paging,
    },
};
use axum::extract::{Query, State};
use sea_orm::Order;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub(crate) struct QueryParams {
    #[serde(flatten, default)]
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

pub async fn page(
    Query(params): Query<QueryParams>,
    State(state): State<ApiState>,
) -> Response<Page<store::Extension>> {
    if params.r#type.is_empty() || params.r#type != "plugin" && params.r#type != "deck" {
        return Response::err("invalid type");
    }
    let result = Extension::page_extensions(
        &state.db,
        params.paging,
        params.r#type,
        params.author,
        params.name,
        params.sort_by.map(|v| match v.as_str() {
            "updateTime" => SortBy::UpdateTime,
            _ => SortBy::DownloadNum,
        }),
        params.order.map(|v| match v.as_str() {
            order if order.to_lowercase() == "asc" => Order::Desc,
            _ => Order::Asc,
        }),
    )
    .await
    .map(|page| page.map(store::Extension::from));

    Response::from(result)
}
