use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{entity::prelude::*, ActiveValue, ConnectionTrait};
use serde::Serialize;

use shared_shared_macro::Dto;

#[derive(Debug, Clone, DeriveEntityModel, Serialize, Default, Dto)]
#[sea_orm(table_name = "tokens")]
#[dto(name(TokenForCreate), columns(user_id, client_id, scopes))]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    #[sea_orm(column_type = "String(StringLen::N(2048))", unique)]
    pub access_token: String,
    #[sea_orm(column_type = "String(StringLen::N(2048))", unique)]
    pub refresh_token: String,
    #[sea_orm(column_type = "Uuid")]
    pub user_id: Uuid,
    #[sea_orm(column_type = "String(StringLen::N(250))")]
    pub client_id: String,
    #[sea_orm(column_type = "JsonBinary")]
    pub scopes: Vec<String>,
    pub access_token_expires_at: DateTime,
    pub refresh_token_expires_at: DateTime,
    pub revoked_at: Option<DateTime>,
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
            // Set the expiration times for access and refresh tokens
            self.access_token_expires_at =
                ActiveValue::Set(current_time + chrono::Duration::days(1));
            self.refresh_token_expires_at =
                ActiveValue::Set(current_time + chrono::Duration::weeks(1));
            self.revoked_at = ActiveValue::Set(None);
        }
        Ok(self)
    }
}
