use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{entity::prelude::*, ActiveValue, ConnectionTrait};
use serde::Serialize;

use shared_shared_macro::Dto;

#[derive(Debug, Clone, DeriveEntityModel, Serialize, Default, Dto)]
#[sea_orm(table_name = "clients")]
#[dto(
    name(ClientForCreate),
    columns(client_secret, name, description, redirect_uris, allowed_grants)
)]
#[dto(
    name(ClientForUpdate),
    columns(client_secret, name, description, redirect_uris, allowed_grants),
    option
)]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    #[sea_orm(column_type = "String(StringLen::N(256))")]
    pub client_secret: String,
    #[sea_orm(column_type = "String(StringLen::N(250))")]
    pub name: String,
    #[sea_orm(column_type = "String(StringLen::N(250))", nullable)]
    pub description: String,
    #[sea_orm(column_type = "String(StringLen::N(512))", array)]
    pub redirect_uris: Vec<String>,
    #[sea_orm(column_type = "String(StringLen::N(512))", array)]
    pub allowed_grants: Vec<String>,
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
        }
        Ok(self)
    }
}
