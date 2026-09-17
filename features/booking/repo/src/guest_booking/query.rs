use shared_shared_data_core::{
    filter::FilterEnum,
    order::Order,
    paging::{Pagination, QueryResult},
    query_params::QueryParams,
};
use shared_shared_data_error::app::AppError;
use shared_shared_macro::Query;

use features_booking_entities::guest_booking::{ActiveModel, Column, Entity, Model, ModelOptionDto};
use features_booking_entities::guest_booking_item::Entity as GuestBookingItemEntity;
use features_booking_model::guest_booking::GuestBookingData;

#[derive(Query)]
#[query(key_type(Uuid))]
#[query_filter(column_name(Column))]
#[query_related(entity(GuestBookingItemEntity), field(items), name("items"))]
struct GuestBookingQueryManager;

pub struct GuestBookingQuery;

impl GuestBookingQuery {
    pub async fn get_guest_booking_by_id(
        guest_booking_id: Uuid,
        query_params: &QueryParams,
    ) -> Result<GuestBookingData, AppError> {
        let includes = query_params.includes();
        let model = GuestBookingQueryManager::get_by_id_uuid_with_related_entities(
            guest_booking_id,
            &includes,
            &vec![],
        )
        .await?;
        Ok(model.into())
    }

    pub async fn get_guest_bookings<'a>(
        pagination: &Pagination,
        order: &Order,
        filters: &FilterCondition,
        query_params: &QueryParams,
    ) -> Result<QueryResult<GuestBookingData>, AppError> {
        let includes = query_params.includes();
        let result = if includes.is_empty() {
            GuestBookingQueryManager::filter(pagination, order, filters).await?
        } else {
            GuestBookingQueryManager::filter_with_related_entities(
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

    /// Load a guest booking row within an existing transaction (raw entity model).
    /// Used by the promotion flow to read + validate current state atomically.
    pub async fn get_by_id_with_txn(
        guest_booking_id: Uuid,
        txn: &impl sea_orm::ConnectionTrait,
    ) -> Result<Option<Model>, sea_orm::DbErr> {
        use sea_orm::EntityTrait;
        Entity::find_by_id(guest_booking_id).one(txn).await
    }

    /// Count active PENDING guest bookings for an email created at/after `since`.
    /// Used to throttle abuse of the public create endpoint (per-email rate cap).
    pub async fn count_active_pending_by_email_since(
        guest_email: &str,
        since: chrono::NaiveDateTime,
    ) -> Result<u64, AppError> {
        use sea_orm::{ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter};
        let db = shared_shared_config::db::DB_READ
            .get()
            .expect("DB_READ not initialized");
        Entity::find()
            .filter(Column::GuestEmail.eq(guest_email))
            .filter(Column::Status.eq(
                features_booking_model::guest_booking::GuestBookingStatus::PENDING,
            ))
            .filter(Column::CreatedAt.gte(since))
            .count(db.as_ref())
            .await
            .map_err(|_| AppError::Unknown)
    }
}
