use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ExtensionTagRelation::Table)
                    .if_not_exists()
                    .col(pk_auto(ExtensionTagRelation::Id))
                    .col(integer(ExtensionTagRelation::ExtensionId))
                    .col(integer(ExtensionTagRelation::TagId))
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .table(ExtensionTagRelation::Table)
                    .name("idx_extension_id")
                    .col(ExtensionTagRelation::ExtensionId)
                    .if_not_exists()
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .table(ExtensionTagRelation::Table)
                    .name("idx_tag_id")
                    .col(ExtensionTagRelation::TagId)
                    .if_not_exists()
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ExtensionTagRelation::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum ExtensionTagRelation {
    Table,
    Id,
    ExtensionId,
    TagId,
}
