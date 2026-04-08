use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{entity::prelude::*, ActiveValue, ConnectionTrait};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use shared_shared_macro::Dto;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, Default, Dto)]
#[sea_orm(table_name = "translation_versions")]
#[dto(
    name(TranslationVersionForCreate),
    columns(key_id, locale, content, version_number, status, created_by)
)]
#[dto(name(TranslationVersionForUpdate), columns(content, status), option)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub key_id: Uuid,
    #[sea_orm(column_type = "String(StringLen::N(10))")]
    pub locale: String,
    #[sea_orm(column_type = "Text")]
    pub content: String,
    pub version_number: i32,
    #[sea_orm(column_type = "String(StringLen::N(20))")]
    pub status: String,

    pub created_by: Uuid,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::translation_key::Entity",
        from = "Column::KeyId",
        to = "super::translation_key::Column::Id"
    )]
    TranslationKey,
}

impl Related<super::translation_key::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TranslationKey.def()
    }
}

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
        }
        Ok(self)
    }
}
