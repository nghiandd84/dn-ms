use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{entity::prelude::*, ActiveValue, ConnectionTrait};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use shared_shared_macro::Dto;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, Default, Dto)]
#[sea_orm(table_name = "reservations")]
#[dto(
    name(ReservationForCreate),
    columns(seat_id, event_id, user_id, expires_at, status)
)]
#[dto(
    name(ReservationForUpdate),
    columns(seat_id, event_id, user_id, expires_at, status),
    option
)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub seat_id: Uuid,
    pub event_id: Uuid,
    pub user_id: Uuid,
    pub expires_at: DateTime,
    pub status: String,             // ENUM('ACTIVE','CONFIRMED','EXPIRED','CANCELLED') DEFAULT 'ACTIVE'
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
