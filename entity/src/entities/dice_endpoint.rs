//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.0

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "dice_endpoint")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub dice_id: i32,
    pub uid: String,
    pub platform: String,
    pub invite_url: String,
    pub last_tick_time: i64,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: Option<DateTimeWithTimeZone>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::dice_info::Entity",
        from = "Column::DiceId",
        to = "super::dice_info::Column::Id"
    )]
    DiceInfo,
}

impl ActiveModelBehavior for ActiveModel {}
