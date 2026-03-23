use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{entity::prelude::*, ActiveValue, ConnectionTrait};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use shared_shared_macro::Dto;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, Default, Dto)]
#[sea_orm(table_name = "fee_configurations")]
#[dto(
    name(FeeConfigurationForCreate),
    columns(
        merchant_id,
        pricing_model,
        percentage_rate,
        fixed_amount,
        min_fee,
        max_fee,
        tier_config,
        effective_from,
        effective_to
    )
)]
#[dto(
    name(FeeConfigurationForUpdate),
    columns(
        merchant_id,
        pricing_model,
        percentage_rate,
        fixed_amount,
        min_fee,
        max_fee,
        tier_config,
        effective_from,
        effective_to
    ),
    option
)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    #[sea_orm(column_type = "String(StringLen::N(50))")]
    pub merchant_id: String,
    #[sea_orm(column_type = "String(StringLen::N(20))")]
    pub pricing_model: String, // PERCENTAGE, FIXED, TIERED, HYBRID
    pub percentage_rate: f32,
    pub fixed_amount: f32,
    pub min_fee: f32,
    pub max_fee: f32,
    pub tier_config: Json, // JSONB for tiered pricing
    // effective_from can be null for currently active configuration
    pub effective_from: DateTime,
    // effective_to can be null for currently active configuration
    pub effective_to: DateTime,
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
