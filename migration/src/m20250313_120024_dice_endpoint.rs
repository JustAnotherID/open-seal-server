use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(DiceEndpoint::Table)
                    .if_not_exists()
                    .col(pk_auto(DiceEndpoint::Id))
                    .col(integer(DiceEndpoint::DiceId))
                    .col(string(DiceEndpoint::Uid))
                    .col(string(DiceEndpoint::Platform))
                    .col(string(DiceEndpoint::InviteUrl))
                    .col(big_unsigned(DiceEndpoint::LastTickTime))
                    .col(timestamp_with_time_zone(DiceEndpoint::CreatedAt))
                    .col(timestamp_with_time_zone_null(DiceEndpoint::UpdatedAt))
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .table(DiceEndpoint::Table)
                    .name("idx_dice_id")
                    .col(DiceEndpoint::DiceId)
                    .if_not_exists()
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(DiceEndpoint::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum DiceEndpoint {
    Table,
    Id,
    DiceId,
    Uid,
    Platform,
    InviteUrl,
    LastTickTime,
    CreatedAt,
    UpdatedAt,
}
