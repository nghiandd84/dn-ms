use shared_shared_data_core::{
    filter::FilterEnum,
    order::Order,
    paging::{Pagination, QueryResult},
    query_params::QueryParams,
};
use shared_shared_data_error::app::AppError;
use shared_shared_macro::Query;

use features_booking_entities::booking::{ActiveModel, Column, Entity, ModelOptionDto};
use features_booking_entities::booking_approval::Entity as BookingApprovalEntity;
use features_booking_entities::booking_capacity::Entity as BookingCapacityEntity;
use features_booking_entities::booking_dispatch::Entity as BookingDispatchEntity;
use features_booking_entities::booking_item::Entity as BookingItemEntity;
use features_booking_entities::booking_queue::Entity as BookingQueueEntity;
use features_booking_entities::booking_recurrence::Entity as BookingRecurrenceEntity;
use features_booking_entities::booking_window::Entity as BookingWindowEntity;
use features_booking_model::booking::BookingData;

#[derive(Query)]
#[query(key_type(Uuid))]
#[query_filter(column_name(Column))]
#[query_related(entity(BookingItemEntity), field(items), name("items"))]
#[query_related(entity(BookingWindowEntity), field(window), name("window"))]
#[query_related(entity(BookingCapacityEntity), field(capacity), name("capacity"))]
#[query_related(entity(BookingRecurrenceEntity), field(recurrence), name("recurrence"))]
#[query_related(entity(BookingApprovalEntity), field(approval), name("approval"))]
#[query_related(entity(BookingQueueEntity), field(queue), name("queue"))]
#[query_related(entity(BookingDispatchEntity), field(dispatch), name("dispatch"))]
struct BookingQueryManager;

pub struct BookingQuery;

impl BookingQuery {
    pub async fn get_booking_by_id(
        booking_id: Uuid,
        query_params: &QueryParams,
    ) -> Result<BookingData, AppError> {
        let includes = query_params.includes();
        let model = BookingQueryManager::get_by_id_uuid_with_related_entities(
            booking_id,
            &includes,
            &vec![],
        )
        .await?;
        Ok(model.into())
    }

    pub async fn get_bookings<'a>(
        pagination: &Pagination,
        order: &Order,
        filters: &FilterCondition,
        query_params: &QueryParams,
    ) -> Result<QueryResult<BookingData>, AppError> {
        let includes = query_params.includes();
        let result = if includes.is_empty() {
            BookingQueryManager::filter(pagination, order, filters).await?
        } else {
            BookingQueryManager::filter_with_related_entities(
                pagination, order, filters, &includes, &vec![],
            )
            .await?
        };
        let mapped_result = QueryResult {
            total_page: result.total_page,
            result: result.result.into_iter().map(|m| m.into()).collect(),
        };
        Ok(mapped_result)
    }
}
