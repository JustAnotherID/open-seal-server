use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(FileInfo::Table)
                    .if_not_exists()
                    .col(pk_auto(FileInfo::Id))
                    .col(string(FileInfo::Name))
                    .col(string(FileInfo::UniformId))
                    .col(string(FileInfo::Key))
                    .col(string(FileInfo::Secret))
                    .col(blob(FileInfo::Content))
                    .col(timestamp_with_time_zone(FileInfo::CreatedAt))
                    .col(timestamp_with_time_zone_null(FileInfo::UpdatedAt))
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .table(FileInfo::Table)
                    .name("idx_name_uniform_id")
                    .unique()
                    .col(FileInfo::Name)
                    .col(FileInfo::UniformId)
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .table(FileInfo::Table)
                    .name("idx_key")
                    .unique()
                    .col(FileInfo::Key)
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .table(FileInfo::Table)
                    .name("idx_key_secret")
                    .unique()
                    .col(FileInfo::Key)
                    .col(FileInfo::Secret)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(FileInfo::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum FileInfo {
    Table,
    Id,
    Name,
    UniformId,
    Key,
    Secret,
    Content,
    CreatedAt,
    UpdatedAt,
}
