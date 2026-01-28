use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{entity::prelude::*, ActiveValue, ConnectionTrait};
use serde::{Deserialize, Serialize};
use shared_shared_macro::Dto;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, Default, Dto)]
#[sea_orm(table_name = "projects")]
#[dto(
    name(ProjectForCreate),
    columns(name, user_id, api_key, default_locale)
)]
#[dto(name(ProjectForUpdate), columns(name, default_locale), option)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub user_id: Uuid,
    #[sea_orm(column_type = "String(StringLen::N(255))")]
    pub name: String,
    #[sea_orm(column_type = "String(StringLen::N(64))", unique)]
    pub api_key: String,
    #[sea_orm(column_type = "String(StringLen::N(10))", nullable)]
    pub default_locale: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::translation_key::Entity")]
    TranslationKey,
}

#[async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(mut self, _db: &C, insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        let current_time = Utc::now().naive_utc();
        if insert {
            self.created_at = ActiveValue::Set(current_time);
            self.updated_at = ActiveValue::Set(current_time);
        } else {
            self.updated_at = ActiveValue::Set(current_time);
        }
        Ok(self)
    }
}
