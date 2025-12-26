use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{entity::prelude::*, ActiveValue, ConnectionTrait};
use serde::{Deserialize, Serialize};
use shared_shared_macro::Dto;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, Default, Dto)]
#[sea_orm(table_name = "active_codes")]
#[dto(name(ActiveCodeForCreate), columns(code, expiration_time))]
#[dto(name(ActiveCodeForUpdate), columns(is_used), option)]

pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    #[sea_orm(column_type = "String(StringLen::N(250))", unique)]
    pub code: String,
    pub is_used: bool,
    pub expiration_time: DateTime,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

#[async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(mut self, _db: &C, insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        let current_time = Utc::now().naive_utc();
        self.updated_at = ActiveValue::Set(current_time);
        if insert {
            self.created_at = ActiveValue::Set(current_time);
            self.is_used = ActiveValue::Set(false);
        }
        Ok(self)
    }
}
