use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{entity::prelude::*, ActiveValue, ConnectionTrait};
use serde::Serialize;

use shared_shared_macro::Dto;

#[derive(Debug, Clone, DeriveEntityModel, Serialize, Default, Dto)]
#[sea_orm(table_name = "authentication_requests")]
#[dto(
    name(AuthenticationRequestForCreate),
    columns(client_id, scopes, response_type, redirect_uri, state)
)]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    #[sea_orm(column_type = "Uuid")]
    pub client_id: Uuid,
    #[sea_orm(column_type = "String(StringLen::N(1024))", array)]
    pub scopes: Vec<String>,
    #[sea_orm(column_type = "String(StringLen::N(128))", unique)]
    pub response_type: String,
    #[sea_orm(column_type = "String(StringLen::N(6020))", unique)]
    pub state: String,
    pub redirect_uri: String,
    pub expires_at: DateTime,
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
            // Set the expiration time to 1 minute from now
            self.expires_at = ActiveValue::Set(current_time + chrono::Duration::minutes(1));
        }
        Ok(self)
    }
}
