use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{entity::prelude::*, ActiveValue, ConnectionTrait};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use shared_shared_macro::Dto;

use crate::lookup_item;

use crate::lookup_item::Model as LookupModel;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, Default, Dto)]
#[sea_orm(table_name = "lookup_types")]
#[dto(name(LookupTypeForCreate), columns(tenant_id, code, name, description))]
#[dto(
    name(LookupTypeForUpdate),
    columns(code, name, description, is_active),
    option
)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub tenant_id: String,
    pub code: String,
    pub name: String,
    pub description: String,
    pub is_active: bool,
    pub created_at: DateTime,
    pub updated_at: DateTime,

    #[sea_orm(ignore)]
    pub items: Vec<LookupModel>,
}


#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "lookup_item::Entity")]
    LookupItem,
}

impl Related<lookup_item::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::LookupItem.def()
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
