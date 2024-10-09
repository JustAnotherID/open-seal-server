use crate::db::{Page, Paging};
use anyhow::Error;
use entity::entities::prelude::Extension;
use sea_orm::DatabaseConnection;

pub async fn page_extensions(
    db: &DatabaseConnection,
    paging: Paging,
) -> Result<Page<Extension>, Error> {
    todo!()
}
