use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{entity::prelude::*, ActiveValue, ConnectionTrait};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use shared_shared_macro::Dto;

use super::tag_group;
use super::tag_group::Model as TagGroupModel;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, Default, Dto)]
#[sea_orm(table_name = "tags")]
#[dto(
    name(TagForCreate),
    columns(
        tenant_id,
        tag_group_id,
        name,
        slug,
        color,
        description,
        alias_of,
        is_active,
        sort_order
    )
)]
#[dto(
    name(TagForUpdate),
    columns(
        tag_group_id,
        name,
        slug,
        color,
        description,
        alias_of,
        is_active,
        sort_order
    ),
    option
)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub tenant_id: String,
    pub tag_group_id: Uuid,
    pub name: String,
    pub slug: String,
    pub color: String,
    pub description: String,
    pub alias_of: Option<Uuid>,
    pub is_active: bool,
    pub sort_order: i32,
    pub usage_count: i32,
    pub created_at: DateTime,
    pub updated_at: DateTime,

    #[sea_orm(ignore)]
    pub tag_group: Vec<TagGroupModel>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "tag_group::Entity",
        from = "Column::TagGroupId",
        to = "tag_group::Column::Id"
    )]
    TagGroup,
    #[sea_orm(
        belongs_to = "Entity",
        from = "Column::AliasOf",
        to = "Column::Id"
    )]
    AliasTag,
}

impl Related<tag_group::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TagGroup.def()
    }
}

#[async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(mut self, _db: &C, insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        let current_time = Utc::now().naive_utc();
        if insert {
            self.id = ActiveValue::Set(Uuid::new_v4());
            self.usage_count = ActiveValue::Set(0);
            self.created_at = ActiveValue::Set(current_time);
        }
        self.updated_at = ActiveValue::Set(current_time);
        Ok(self)
    }
}
