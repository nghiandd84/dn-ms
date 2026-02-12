use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{entity::prelude::*, ActiveValue, ConnectionTrait};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use shared_shared_macro::Dto;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, Default, Dto)]
#[sea_orm(table_name = "events")]
#[dto(
    name(EventForCreate),
    columns(
        event_name,
        event_date,
        venue_name,
        total_seats,
        status,
        sale_start_time
    )
)]
#[dto(
    name(EventForUpdate),
    columns(
        event_name,
        event_date,
        venue_name,
        total_seats,
        status,
        sale_start_time
    ),
    option
)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    #[sea_orm(column_type = "String(StringLen::N(255))")]
    pub event_name: String,
    pub event_date: DateTime,
    #[sea_orm(column_type = "String(StringLen::N(255))", nullable)]
    pub venue_name: String,
    pub total_seats: i32,
    #[sea_orm(column_type = "String(StringLen::N(50))")]
    pub status: String,
    pub sale_start_time: DateTime,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

// #[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
// #[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "event_status")]
// pub enum EventStatus {
//     #[sea_orm(string_value = "UPCOMING")]
//     Upcoming,
//     #[sea_orm(string_value = "ON_SALE")]
//     OnSale,
//     #[sea_orm(string_value = "SOLD_OUT")]
//     SoldOut,
//     #[sea_orm(string_value = "CANCELLED")]
//     Cancelled,
// }

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
