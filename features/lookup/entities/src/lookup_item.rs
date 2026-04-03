use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{entity::prelude::*, ActiveValue, ConnectionTrait};
use serde::{Deserialize, Serialize};
use serde_json::Value as Json;
use uuid::Uuid;

use shared_shared_macro::Dto;

use super::lookup_type;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, Default, Dto)]
#[sea_orm(table_name = "lookup_items")]
#[dto(
    name(LookupItemForCreate),
    columns(
        lookup_type_id,
        code,
        name,
        url,
        query_param_one,
        query_param_two,
        tenants,
        is_active,
        sort_order
    )
)]
#[dto(
    name(LookupItemForUpdate),
    columns(
        code,
        name,
        url,
        query_param_one,
        query_param_two,
        tenants,
        is_active,
        sort_order
    ),
    option
)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub lookup_type_id: Uuid,
    pub code: String,
    pub name: String,
    pub url: String,
    pub query_param_one: String,
    pub query_param_two: String,
    pub tenants: Vec<String>,
    pub is_active: bool,
    pub sort_order: i32,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "lookup_type::Entity",
        from = "Column::LookupTypeId",
        to = "lookup_type::Column::Id"
    )]
    LookupType,
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
        }
        self.updated_at = ActiveValue::Set(current_time);
        Ok(self)
    }
}
