use sea_orm::{ActiveModelTrait, ConnectionTrait, DbErr, EntityTrait, IntoActiveModel};
use uuid::Uuid;

use shared_shared_config::db::DB_READ;
use shared_shared_data_error::app::AppError;

use features_booking_entities::booking_approval::{
    BookingApprovalForCreateDto, Entity, Model, ModelOptionDto,
};
use features_booking_model::booking_approval::BookingApprovalData;

pub struct BookingApprovalMutation;

impl BookingApprovalMutation {
    /// Insert an approval row within a transaction. PK is `booking_id`.
    pub async fn create_with_txn(
        data: BookingApprovalForCreateDto,
        txn: &impl ConnectionTrait,
    ) -> Result<Uuid, DbErr> {
        let model: Model = data.into();
        let booking_id = model.booking_id;
        let active_model = model.into_active_model();
        active_model.insert(txn).await?;
        Ok(booking_id)
    }
}

pub struct BookingApprovalQuery;

impl BookingApprovalQuery {
    pub async fn get_by_booking_id(
        booking_id: Uuid,
    ) -> Result<Option<BookingApprovalData>, AppError> {
        let db = DB_READ.get().expect("DB_READ not initialized");
        let model = Entity::find_by_id(booking_id)
            .one(db.as_ref())
            .await
            .map_err(|_| AppError::Unknown)?;
        Ok(model.map(|m| {
            let opt: ModelOptionDto = m.into();
            opt.into()
        }))
    }
}
