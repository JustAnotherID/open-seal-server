use crate::service::{Page, Paging};
use anyhow::Error;
use entity::entities::{extension, prelude};
use sea_orm::{
    ColumnTrait, DatabaseConnection, EntityTrait, Order, PaginatorTrait, QueryFilter, QueryOrder,
    QueryTrait,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub(crate) enum SortBy {
    DownloadNum,
    UpdateTime,
    Other(String),
}

pub(crate) struct Extension {}

impl Extension {
    pub(crate) async fn page_extensions(
        db: &DatabaseConnection,
        paging: Paging,
        r#type: String,
        author: Option<String>,
        name: Option<String>,
        sort_by: Option<SortBy>,
        order: Option<Order>,
    ) -> Result<Page<extension::Model>, Error> {
        let p = prelude::Extension::find()
            .filter(extension::Column::Type.eq(r#type))
            .apply_if(author, |q, v| {
                q.filter(extension::Column::Authors.contains(v))
            })
            .apply_if(name, |q, v| q.filter(extension::Column::Name.contains(v)))
            .apply_if(sort_by, |q, v| {
                let order = order.unwrap_or(Order::Asc);
                match v {
                    SortBy::UpdateTime => q.order_by(extension::Column::UpdateTime, order),
                    _ => q.order_by(extension::Column::DownloadNum, order),
                }
            })
            .paginate(db, 20);

        let total = p.num_items().await?;
        let before_nums = (paging.page_num - 1) * paging.page_size;
        if total <= before_nums {
            return Ok(Page::new(paging.page_num, 0, vec![], false));
        }

        let data: Vec<extension::Model> = p.fetch_page(paging.page_num - 1).await?;
        let has_next = total > before_nums + (data.len() as u64);
        Ok(Page::new(
            paging.page_num,
            data.len() as u64,
            data,
            has_next,
        ))
    }
}
