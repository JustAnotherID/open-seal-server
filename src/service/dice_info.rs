use crate::service::Paging;
use anyhow::{anyhow, Error};
use chrono::Utc;
use entity::entities::{dice_endpoint, dice_info};
use nid::{alphabet::Base62Alphabet, Nanoid};
use sea_orm::{
    prelude::Expr, ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection,
    EntityTrait, QueryFilter, TransactionTrait,
};

pub(crate) struct DiceInfo {}

impl DiceInfo {
    pub(crate) async fn list(
        _db: &DatabaseConnection,
        _paging: Paging,
    ) -> Result<Vec<dice_info::Model>, Error> {
        todo!()
    }
    pub(crate) async fn register_or_update_dice(
        db: &DatabaseConnection,
        openid: &Option<String>,
        name: &str,
        brief: &str,
        note: &str,
        avatar: &str,
        version: &str,
    ) -> Result<dice_info::Model, Error> {
        let openid: Nanoid<22, Base62Alphabet> = match openid {
            // 接收到一个 openid
            Some(openid) => {
                let exists = DiceInfo::find_dice(db, openid).await;
                match exists {
                    Some(exists) => {
                        // 说明是更新
                        return DiceInfo::update_dice(
                            db, exists, name, brief, note, avatar, version,
                        )
                        .await;
                    }
                    None => {
                        // 接收到一个 openid，但没有对应记录，如果 openid 符合当前使用的 nanoid 格式的话，用这个去注册，否则报错
                        Nanoid::try_from_str(openid)?
                    }
                }
            }
            // 无 openid，直接注册
            None => Nanoid::new(),
        };
        let now = Utc::now().fixed_offset();
        let record = dice_info::ActiveModel {
            openid: Set(openid.to_string()),
            name: Set(name.to_owned()),
            brief: Set(brief.to_owned()),
            note: Set(note.to_owned()),
            avatar: Set(avatar.to_owned()),
            version: Set(version.to_owned()),
            last_tick_time: Set(now.timestamp()),
            created_at: Set(now),
            updated_at: Set(Some(now)),
            update_tick_count: Set(0),
            ..Default::default()
        };
        let model = record.insert(db).await?;
        Ok(model)
    }

    pub(crate) async fn tick_update(
        db: &DatabaseConnection,
        openid: &str,
        endpoint_uid_list: Vec<String>,
    ) -> Result<(), Error> {
        let exists = dice_info::Entity::find()
            .filter(dice_info::Column::Openid.eq(openid))
            .one(db)
            .await?
            .ok_or(anyhow!("public dice not found"))?;
        let now = Utc::now().fixed_offset();
        let dice_tick_count = exists.update_tick_count;

        let mut update: dice_info::ActiveModel = exists.into();
        update.updated_at = Set(Some(now));
        update.update_tick_count = Set(dice_tick_count + 1);
        let txn = db.begin().await?;
        update.update(&txn).await?;
        if !endpoint_uid_list.is_empty() {
            dice_endpoint::Entity::update_many()
                .col_expr(dice_endpoint::Column::LastTickTime, Expr::value(now))
                .col_expr(dice_endpoint::Column::UpdatedAt, Expr::value(now))
                .filter(dice_endpoint::Column::Uid.is_in(endpoint_uid_list))
                .exec(&txn)
                .await?;
        }
        txn.commit().await?;
        Ok(())
    }

    async fn find_dice(db: &DatabaseConnection, openid: &String) -> Option<dice_info::Model> {
        dice_info::Entity::find()
            .filter(dice_info::Column::Openid.eq(openid))
            .one(db)
            .await
            .ok()
            .flatten()
    }

    async fn update_dice(
        db: &DatabaseConnection,
        exists: dice_info::Model,
        name: &str,
        brief: &str,
        note: &str,
        avatar: &str,
        version: &str,
    ) -> Result<dice_info::Model, Error> {
        let now = Utc::now().fixed_offset();
        let mut update: dice_info::ActiveModel = exists.into();
        if !name.is_empty() {
            update.name = Set(name.to_owned());
        }
        if !brief.is_empty() {
            update.brief = Set(brief.to_owned());
        }
        if !note.is_empty() {
            update.note = Set(note.to_owned());
        }
        if !avatar.is_empty() {
            update.avatar = Set(avatar.to_owned());
        }
        if !version.is_empty() {
            update.version = Set(version.to_owned());
        }
        update.updated_at = Set(Some(now));
        let model: dice_info::Model = update.update(db).await?;
        Ok(model)
    }
}

#[cfg(test)]
mod tests {
    use sea_orm::PaginatorTrait;

    use crate::service::conn::build_test_db;

    use super::*;

    #[tokio::test]
    async fn test_register_or_update_dice() {
        let db = build_test_db().await;
        let dice = DiceInfo::register_or_update_dice(
            &db,
            &None,
            "Dice1",
            "this is Dice1",
            "",
            "",
            "1.4.6",
        )
        .await
        .unwrap();
        assert_eq!(dice.openid.len(), 22);

        let openid = dice.openid.clone();
        let updated = DiceInfo::register_or_update_dice(
            &db,
            &Some(dice.openid),
            "Dice1",
            "this is Dice1",
            "",
            "",
            "1.4.6",
        )
        .await
        .unwrap();
        assert_eq!(updated.openid, openid);

        let another = DiceInfo::register_or_update_dice(
            &db,
            &None,
            "Dice2",
            "this is Dice2",
            "",
            "",
            "1.4.6",
        )
        .await
        .unwrap();
        assert_eq!(another.openid.len(), 22);

        let total = dice_info::Entity::find().count(&db).await.unwrap();
        assert_eq!(total, 2)
    }
}
