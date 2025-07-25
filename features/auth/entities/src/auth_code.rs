use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{entity::prelude::*, ActiveValue, ConnectionTrait};
use serde::Serialize;

use shared_shared_macro::Dto;

#[derive(Debug, Clone, DeriveEntityModel, Serialize, Default, Dto)]
#[sea_orm(table_name = "auth_codes")]
#[dto(
    name(AuthCodeForCreate),
    columns(user_id, client_id, scopes, redirect_uri)
)]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    #[sea_orm(column_type = "String(StringLen::N(1204))", unique)]
    pub code: String,
    #[sea_orm(column_type = "Uuid")]
    pub user_id: Uuid,
    #[sea_orm(column_type = "Uuid")]
    pub client_id: Uuid,
    #[sea_orm(column_type = "String(StringLen::N(128))", array)]
    pub scopes: Vec<String>,
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
            self.code = ActiveValue::Set(generate_auth_code());
            self.created_at = ActiveValue::Set(current_time);
            // Set the expiration time to 1 minute from now
            self.expires_at = ActiveValue::Set(current_time + chrono::Duration::minutes(1));
        }
        Ok(self)
    }
}

fn generate_auth_code() -> String {
    use rand::distributions::Alphanumeric;
    use rand::Rng;

    let code_length = 64; // Length of the auth code
    let code: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(code_length)
        .map(char::from)
        .collect();
    code
}
