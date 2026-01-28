use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{entity::prelude::*, ActiveValue};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use shared_shared_macro::Dto;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, Default, Dto)]
#[sea_orm(table_name = "key_tags")]
#[dto(name(KeyTagForCreate), columns(key_id, tag_id))]
#[dto(name(KeyTagForUpdate), columns(key_id, tag_id), option)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub key_id: Uuid,
    #[sea_orm(primary_key, auto_increment = false)]
    pub tag_id: Uuid,
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
    #[sea_orm(
        belongs_to = "super::tag::Entity",
        from = "Column::TagId",
        to = "super::tag::Column::Id"
    )]
    Tag,
}


impl Related<super::translation_key::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TranslationKey.def()
    }
}

impl Related<super::tag::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Tag.def()
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
