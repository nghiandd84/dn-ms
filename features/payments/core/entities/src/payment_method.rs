use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{entity::prelude::*, ActiveValue, ConnectionTrait};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use shared_shared_macro::Dto;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, Default, Dto)]
#[sea_orm(table_name = "payment_methods")]
#[dto(
    name(PaymentMethodForCreate),
    columns(
        display_name,
        provider_name,
        provider_config,
        supported_countries,
        supported_currencies,
        priority,
        is_active,
        fee_percentage,
        icon_url
    )
)]
#[dto(
    name(PaymentMethodForUpdate),
    columns(
        display_name,
        provider_name,
        provider_config,
        supported_countries,
        supported_currencies,
        priority,
        is_active,
        fee_percentage,
        icon_url
    ),
    option
)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub display_name: String,
    pub provider_name: String,
    pub provider_config: Json,
    #[sea_orm(column_type = "String(StringLen::N(2))", array)]
    pub supported_countries: Vec<String>,
    #[sea_orm(column_type = "String(StringLen::N(3))", array)]
    pub supported_currencies: Vec<String>,
    pub priority: i32,
    pub is_active: bool,
    pub fee_percentage: f32,
    #[sea_orm(column_type = "Text", nullable)]
    pub icon_url: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
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
