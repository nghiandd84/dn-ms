use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{entity::prelude::*, ActiveValue, ConnectionTrait};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use shared_shared_macro::Dto;

use super::tag;
use super::tag::Model as TagModel;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, Default, Dto)]
#[sea_orm(table_name = "entity_tags")]
#[dto(
    name(EntityTagForCreate),
    columns(tag_id, entity_type, entity_id, tenant_id, tagged_by)
)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub tag_id: Uuid,
    pub entity_type: String,
    pub entity_id: Uuid,
    pub tenant_id: String,
    pub tagged_by: Uuid,
    pub created_at: DateTime,

    #[sea_orm(ignore)]
    pub tag: Vec<TagModel>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "tag::Entity",
        from = "Column::TagId",
        to = "tag::Column::Id"
    )]
    Tag,
}

impl Related<tag::Entity> for Entity {
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
        if insert {
            self.id = ActiveValue::Set(Uuid::new_v4());
            self.created_at = ActiveValue::Set(Utc::now().naive_utc());
        }
        Ok(self)
    }
}
