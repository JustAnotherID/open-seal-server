use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ExtensionTag::Table)
                    .if_not_exists()
                    .col(pk_auto(ExtensionTag::Id))
                    .col(string_uniq(ExtensionTag::Name))
                    .col(text_null(ExtensionTag::Desc))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ExtensionTag::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum ExtensionTag {
    Table,
    Id,
    Name,
    Desc,
}
