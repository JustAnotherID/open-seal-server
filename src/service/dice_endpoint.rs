use std::collections::{HashMap, HashSet};

use anyhow::{anyhow, Error};
use chrono::Utc;
use entity::entities::{dice_endpoint, dice_info};
use itertools::Itertools;
use sea_orm::{
    entity::ColumnTrait, ActiveValue::Set, DatabaseConnection, EntityTrait, QueryFilter,
    TransactionTrait,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct EndpointInfoDTO {
    pub uid: String,
    pub platform: String,
    pub invite_url: String,
}

pub(crate) struct DiceEndpoint {}

impl DiceEndpoint {
    pub(crate) async fn update_dice_endpoint_info(
        db: &DatabaseConnection,
        dice_openid: &str,
        endpoint_list: Vec<EndpointInfoDTO>,
    ) -> Result<(), Error> {
        let dice_id = dice_info::Entity::find()
            .filter(dice_info::Column::Openid.eq(dice_openid))
            .one(db)
            .await?
            .ok_or(anyhow!("dice not found"))?
            .id;

        let exists = dice_endpoint::Entity::find()
            .filter(dice_endpoint::Column::DiceId.eq(dice_id))
            .all(db)
            .await?;
        let exists: HashMap<(String, String), dice_endpoint::Model> = exists
            .into_iter()
            .map(|ep| ((ep.uid.clone(), ep.platform.clone()), ep))
            .collect();

        let mut to_add: Vec<dice_endpoint::ActiveModel> = Vec::new();
        let mut to_update: Vec<dice_endpoint::ActiveModel> = Vec::new();
        let mut processed_keys = HashSet::new();
        let now = Utc::now().fixed_offset();

        for ep in endpoint_list {
            let key = (ep.uid.clone(), ep.platform.clone());
            match exists.get(&key) {
                Some(model) => {
                    to_update.push(dice_endpoint::ActiveModel {
                        id: Set(model.id),
                        invite_url: Set(ep.invite_url),
                        last_tick_time: Set(now.timestamp()),
                        updated_at: Set(Some(now)),
                        ..Default::default()
                    });
                }
                None => {
                    to_add.push(dice_endpoint::ActiveModel {
                        dice_id: Set(dice_id),
                        uid: Set(ep.uid),
                        platform: Set(ep.platform),
                        invite_url: Set(ep.invite_url),
                        last_tick_time: Set(now.timestamp()),
                        created_at: Set(now),
                        ..Default::default()
                    });
                }
            }
            processed_keys.insert(key);
        }

        let txn = db.begin().await?;

        if !to_add.is_empty() {
            dice_endpoint::Entity::insert_many(to_add)
                .exec(&txn)
                .await?;
        }
        if !to_update.is_empty() {
            for update in to_update {
                dice_endpoint::Entity::update(update).exec(&txn).await?;
            }
        }

        let to_delete = exists
            .into_iter()
            .filter(|(k, _)| !processed_keys.contains(k))
            .map(|(_, v)| v.id)
            .collect_vec();
        if !to_delete.is_empty() {
            dice_endpoint::Entity::delete_many()
                .filter(dice_endpoint::Column::Id.is_in(to_delete))
                .exec(&txn)
                .await?;
        }
        txn.commit().await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::service::{
        conn::build_test_db,
        dice_endpoint::{DiceEndpoint, EndpointInfoDTO},
        dice_info::DiceInfo,
    };
    use entity::entities::dice_endpoint;
    use sea_orm::{ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter};

    #[tokio::test]
    async fn test_update_dice_endpoint_info() {
        let db = build_test_db().await;
        let dice = DiceInfo::register_or_update_dice(
            &db,
            &None,
            "Dice",
            "test dice",
            "test note",
            "",
            "1.4.6",
        )
        .await
        .unwrap();

        DiceEndpoint::update_dice_endpoint_info(
            &db,
            &dice.openid,
            vec![
                EndpointInfoDTO {
                    uid: "QQ:114515".to_string(),
                    platform: "QQ".to_string(),
                    invite_url: "".to_string(),
                },
                EndpointInfoDTO {
                    uid: "QQ:1919810".to_string(),
                    platform: "QQ".to_string(),
                    invite_url: "".to_string(),
                },
            ],
        )
        .await
        .unwrap();

        let total = dice_endpoint::Entity::find()
            .filter(dice_endpoint::Column::DiceId.eq(dice.id))
            .count(&db)
            .await
            .unwrap();
        assert_eq!(total, 2);

        DiceEndpoint::update_dice_endpoint_info(
            &db,
            &dice.openid,
            vec![
                EndpointInfoDTO {
                    uid: "QQ:114515".to_string(),
                    platform: "QQ".to_string(),
                    invite_url: "".to_string(),
                },
                EndpointInfoDTO {
                    uid: "QQ:1919810".to_string(),
                    platform: "QQ".to_string(),
                    invite_url: "".to_string(),
                },
                EndpointInfoDTO {
                    uid: "KOOK:foo".to_string(),
                    platform: "KOOK".to_string(),
                    invite_url: "".to_string(),
                },
            ],
        )
        .await
        .unwrap();
        let total = dice_endpoint::Entity::find()
            .filter(dice_endpoint::Column::DiceId.eq(dice.id))
            .count(&db)
            .await
            .unwrap();
        assert_eq!(total, 3);
    }
}
