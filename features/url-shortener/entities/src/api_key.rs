use async_trait::async_trait;
use chrono::{NaiveDateTime as DateTime, Utc};
use sea_orm::{entity::prelude::*, ActiveValue, ConnectionTrait};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use shared_shared_macro::Dto;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, Default, Dto)]
#[sea_orm(table_name = "api_keys")]
#[dto(name(ApiKeyForCreate), columns(user_id, key_hash, name))]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub user_id: Uuid,
    #[sea_orm(column_type = "String(StringLen::N(64))", unique)]
    pub key_hash: String,
    #[sea_orm(column_type = "String(StringLen::N(100))")]
    pub name: String,
    pub is_active: bool,
    pub last_used_at: Option<DateTime>,
    pub created_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

#[async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(mut self, _db: &C, insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        if insert {
            self.id = ActiveValue::Set(Uuid::new_v4());
            self.created_at = ActiveValue::Set(Utc::now().naive_utc());
            if self.is_active.is_not_set() {
                self.is_active = ActiveValue::Set(true);
            }
        }
        Ok(self)
    }
}
