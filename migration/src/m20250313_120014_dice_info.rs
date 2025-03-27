use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(DiceInfo::Table)
                    .if_not_exists()
                    .col(pk_auto(DiceInfo::Id))
                    .col(string(DiceInfo::Openid))
                    .col(string(DiceInfo::Name))
                    .col(text(DiceInfo::Brief))
                    .col(text(DiceInfo::Note))
                    .col(string(DiceInfo::Avatar))
                    .col(string(DiceInfo::Version))
                    .col(big_unsigned(DiceInfo::UpdateTickCount))
                    .col(big_unsigned(DiceInfo::LastTickTime))
                    .col(timestamp_with_time_zone(DiceInfo::CreatedAt))
                    .col(timestamp_with_time_zone_null(DiceInfo::UpdatedAt))
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .table(DiceInfo::Table)
                    .name("idx_openid")
                    .unique()
                    .col(DiceInfo::Openid)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(DiceInfo::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum DiceInfo {
    Table,
    Id,
    Openid,
    Name,
    Brief,
    Note,
    Avatar,
    Version,
    UpdateTickCount,
    LastTickTime,
    CreatedAt,
    UpdatedAt,
}
