use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{entity::prelude::*, ActiveValue, ConnectionTrait};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use shared_shared_macro::Dto;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, Default, Dto)]
#[sea_orm(table_name = "payment_method_limits")]
#[dto(
    name(PaymentMethodLimitForCreate),
    columns(payment_method_id, currency, min_amount, max_amount)
)]
#[dto(
    name(PaymentMethodLimitForUpdate),
    columns(payment_method_id, currency, min_amount, max_amount),
    option
)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub payment_method_id: Uuid,
    #[sea_orm(column_type = "String(StringLen::N(3))")]
    pub currency: String,
    pub min_amount: i64,
    pub max_amount: i64,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::payment_method::Entity",
        from = "Column::PaymentMethodId",
        to = "super::payment_method::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    PaymentMethod,
}

impl Related<super::payment_method::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PaymentMethod.def()
    }
}

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