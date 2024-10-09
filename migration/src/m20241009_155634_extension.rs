use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Extension::Table)
                    .if_not_exists()
                    .col(pk_auto(Extension::Id))
                    .col(string_uniq(Extension::OuterId))
                    .col(string(Extension::Namespace))
                    .col(string(Extension::Key))
                    .col(string(Extension::Version))
                    .col(string(Extension::Type))
                    .col(string(Extension::Ext))
                    .col(string(Extension::Name))
                    .col(json(Extension::Authors))
                    .col(text(Extension::Desc))
                    .col(string(Extension::License))
                    .col(big_unsigned(Extension::ReleaseTime))
                    .col(big_unsigned(Extension::UpdateTime))
                    .col(big_unsigned(Extension::DownloadNum))
                    .col(json_null(Extension::Tags))
                    .col(json_null(Extension::Extra))
                    .col(string_null(Extension::HomePage))
                    .col(string_null(Extension::SealVersion))
                    .col(json_null(Extension::Dependencies))
                    .col(timestamp_with_time_zone(Extension::CreatedAt))
                    .col(timestamp_with_time_zone_null(Extension::UpdatedAt))
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .table(Extension::Table)
                    .name("idx_namespace_key_version")
                    .unique()
                    .col(Extension::Namespace)
                    .col(Extension::Key)
                    .col(Extension::Version)
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .table(Extension::Table)
                    .name("idx_outer_id_download_num")
                    .col(Extension::OuterId)
                    .col(Extension::DownloadNum)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Extension::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Extension {
    Table,
    Id,
    OuterId,
    Namespace,
    Key,
    Version,
    Type,
    Ext,
    Name,
    Authors,
    Desc,
    License,
    ReleaseTime,
    UpdateTime,
    DownloadNum,
    Tags,
    Extra,
    HomePage,
    SealVersion,
    Dependencies,
    CreatedAt,
    UpdatedAt,
}
