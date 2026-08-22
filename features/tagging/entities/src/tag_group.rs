use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{entity::prelude::*, ActiveValue, ConnectionTrait};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use shared_shared_macro::Dto;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, Default, Dto)]
#[sea_orm(table_name = "tag_groups")]
#[dto(
    name(TagGroupForCreate),
    columns(tenant_id, code, name, description, parent_id, is_active, sort_order)
)]
#[dto(
    name(TagGroupForUpdate),
    columns(code, name, description, parent_id, is_active, sort_order),
    option
)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub tenant_id: String,
    pub code: String,
    pub name: String,
    pub description: String,
    pub parent_id: Option<Uuid>,
    pub is_active: bool,
    pub sort_order: i32,
    pub created_at: DateTime,
    pub updated_at: DateTime,

    #[sea_orm(ignore)]
    pub children: Vec<Model>,

    #[sea_orm(ignore)]
    pub parent: Vec<Model>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "Entity")]
    Children,
    #[sea_orm(
        belongs_to = "Entity",
        from = "Column::ParentId",
        to = "Column::Id"
    )]
    Parent,
}

impl Related<Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Children.def()
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
            self.created_at = ActiveValue::Set(current_time);
        }
        self.updated_at = ActiveValue::Set(current_time);
        Ok(self)
    }
}
