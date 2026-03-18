use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{entity::prelude::*, ActiveValue, ConnectionTrait};
use serde::{Deserialize, Serialize};

use shared_shared_macro::Dto;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, Default, Dto)]
#[sea_orm(table_name = "merchants")]
#[dto(
    name(MerchantForCreate),
    columns(business_name, email, phone, business_type, kyc_status, status)
)]
#[dto(
    name(MerchantForUpdate),
    columns(
        business_name,
        email,
        phone,
        business_type,
        kyc_status,
        status,
        kyc_verified_at
    ),
    option
)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    #[sea_orm(column_type = "String(StringLen::N(255))")]
    pub business_name: String,
    #[sea_orm(column_type = "String(StringLen::N(255))")]
    pub email: String,
    #[sea_orm(column_type = "String(StringLen::N(20))")]
    pub phone: String,
    #[sea_orm(column_type = "String(StringLen::N(50))")]
    pub business_type: String, // INDIVIDUAL, COMPANY, PARTNERSHIP
    #[sea_orm(column_type = "String(StringLen::N(20))")]
    pub kyc_status: String, // PENDING, APPROVED, REJECTED
    pub kyc_verified_at: DateTime,
    #[sea_orm(column_type = "String(StringLen::N(20))")]
    pub status: String, // ACTIVE, INACTIVE, SUSPENDED
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
