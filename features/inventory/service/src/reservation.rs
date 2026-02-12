use sea_orm::{DbConn, Iden};
use tracing::debug;
use uuid::Uuid;

use shared_shared_data_core::{
    filter::{FilterEnum, FilterOperator, FilterParam},
    order::Order,
    paging::{Pagination, QueryResult},
};
use shared_shared_data_error::app::AppError;

use features_inventory_entities::reservation::Column;
use features_inventory_model::reservation::{
    ReservationData, ReservationForCreateRequest, ReservationForUpdateRequest,
};
use features_inventory_repo::reservation::{ReservationMutation, ReservationQuery};

pub struct ReservationService {}

impl ReservationService {
    pub async fn create_reservation<'a>(
        db: &'a DbConn,
        reservation_request: ReservationForCreateRequest,
    ) -> Result<Uuid, AppError> {
        let reservation_id =
            ReservationMutation::create_reservation(db, reservation_request.into()).await;
        let id = match reservation_id {
            Ok(id) => id,
            Err(e) => {
                debug!("Error creating reservation: {:?}", e);
                return Err(AppError::Internal(
                    "Failed to create reservation".to_string(),
                ));
            }
        };
        Ok(id)
    }

    pub async fn get_reservation_by_id<'a>(
        db: &'a DbConn,
        reservation_id: Uuid,
    ) -> Result<ReservationData, AppError> {
        ReservationQuery::get_reservation_by_id(db, reservation_id).await
    }

    pub async fn get_reservations_by_status(
        db: &DbConn,
        status: &str,
        pagination: &Pagination,
        order: &Order,
    ) -> Result<QueryResult<ReservationData>, AppError> {
        let status_column = Column::Status.to_string();
        let param: FilterParam<String> = FilterParam {
            name: status_column,
            operator: FilterOperator::Equal,
            value: Some(status.to_string()),
            raw_value: status.to_string(),
        };
        let status_filter = FilterEnum::String(param);
        let filters: Vec<FilterEnum> = vec![status_filter];
        ReservationQuery::get_reservations(db, &pagination, &order, &filters).await
    }

    pub async fn get_reservations<'a>(
        db: &'a DbConn,
        filters: &Vec<FilterEnum>,
        pagination: &Pagination,
        order: &Order,
    ) -> Result<QueryResult<ReservationData>, AppError> {
        ReservationQuery::get_reservations(db, pagination, order, filters).await
    }

    pub async fn update_reservation(
        db: &DbConn,
        reservation_id: Uuid,
        reservation_request: ReservationForUpdateRequest,
    ) -> Result<bool, AppError> {
        let result =
            ReservationMutation::update_reservation(db, reservation_id, reservation_request.into())
                .await;
        match result {
            Ok(success) => Ok(success),
            Err(e) => {
                debug!("Error updating reservation: {:?}", e);
                Err(AppError::Internal(
                    "Failed to update reservation".to_string(),
                ))
            }
        }
    }

    pub async fn delete_reservation(db: &DbConn, reservation_id: Uuid) -> Result<bool, AppError> {
        let result = ReservationMutation::delete_reservation(db, reservation_id).await;
        match result {
            Ok(success) => Ok(success),
            Err(e) => {
                debug!("Error deleting reservation: {:?}", e);
                Err(AppError::Internal(
                    "Failed to delete reservation".to_string(),
                ))
            }
        }
    }
}
