use sea_orm::{DbConn, Iden};
use tracing::{debug, event};
use uuid::Uuid;

use shared_shared_data_core::{
    filter::{FilterEnum, FilterOperator, FilterParam},
    order::Order,
    paging::{Pagination, QueryResult},
};
use shared_shared_data_error::app::AppError;

use features_inventory_entities::seat::Column;
use features_inventory_model::seat::{SeatData, SeatForCreateRequest, SeatForUpdateRequest};
use features_inventory_repo::seat::{SeatMutation, SeatQuery};

pub struct SeatService {}

impl SeatService {
    pub async fn create_seat<'a>(
        db: &'a DbConn,
        seat_request: SeatForCreateRequest,
    ) -> Result<Uuid, AppError> {
        let seat_id = SeatMutation::create_seat(db, seat_request.into()).await;
        let id = match seat_id {
            Ok(id) => id,
            Err(e) => {
                debug!("Error creating seat: {:?}", e);
                return Err(AppError::Internal("Failed to create seat".to_string()));
            }
        };
        Ok(id)
    }

    pub async fn bulk_create_seats<'a>(
        db: &'a DbConn,
        seat_requests: Vec<SeatForCreateRequest>,
    ) -> Result<Vec<Uuid>, AppError> {
        let seat_ids = SeatMutation::bulk_create_seats(
            db,
            seat_requests.into_iter().map(|r| r.into()).collect(),
        )
        .await;
        match seat_ids {
            Ok(ids) => Ok(ids),
            Err(e) => {
                debug!("Error bulk creating seats: {:?}", e);
                Err(AppError::Internal(
                    "Failed to bulk create seats".to_string(),
                ))
            }
        }
    }

    pub async fn get_seat_by_id<'a>(db: &'a DbConn, seat_id: Uuid) -> Result<SeatData, AppError> {
        SeatQuery::get_seat_by_id(db, seat_id).await
    }

    pub async fn get_seats_by_status(
        db: &DbConn,
        status: &str,
        pagination: &Pagination,
        order: &Order,
    ) -> Result<QueryResult<SeatData>, AppError> {
        let status_column = Column::Status.to_string();
        let param: FilterParam<String> = FilterParam {
            name: status_column,
            operator: FilterOperator::Equal,
            value: Some(status.to_string()),
            raw_value: status.to_string(),
        };
        let status_filter = FilterEnum::String(param);
        let filters: Vec<FilterEnum> = vec![status_filter];
        SeatQuery::get_seats(db, &pagination, &order, &filters).await
    }

    pub async fn get_seats<'a>(
        db: &'a DbConn,
        filters: &Vec<FilterEnum>,
        pagination: &Pagination,
        order: &Order,
    ) -> Result<QueryResult<SeatData>, AppError> {
        SeatQuery::get_seats(db, pagination, order, filters).await
    }

    pub async fn update_seat(
        db: &DbConn,
        seat_id: Uuid,
        seat_request: SeatForUpdateRequest,
    ) -> Result<bool, AppError> {
        let result = SeatMutation::update_seat(db, seat_id, seat_request.into()).await;
        match result {
            Ok(success) => Ok(success),
            Err(e) => {
                debug!("Error updating seat: {:?}", e);
                Err(AppError::Internal("Failed to update seat".to_string()))
            }
        }
    }

    pub async fn delete_seat(db: &DbConn, seat_id: Uuid) -> Result<bool, AppError> {
        let result = SeatMutation::delete_seat(db, seat_id).await;
        match result {
            Ok(success) => Ok(success),
            Err(e) => {
                debug!("Error deleting seat: {:?}", e);
                Err(AppError::Internal("Failed to delete seat".to_string()))
            }
        }
    }
}
