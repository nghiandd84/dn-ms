use uuid::Uuid;

use shared_shared_data_core::{
    filter::FilterEnum,
    order::Order,
    paging::{Pagination, QueryResult},
};
use shared_shared_data_error::app::AppError;
use shared_shared_macro::Query;

use features_payments_core_entities::payment::{ActiveModel, Column, Entity, ModelOptionDto};
use features_payments_core_model::payment::PaymentData;

#[derive(Query)]
#[query(key_type(Uuid))]
#[query_filter(column_name(Column))]
struct PaymentQueryManager;

impl PaymentQueryManager {
    fn build_filter_condition(filters: &Vec<FilterEnum>) -> Condition {
        let mut condition = Condition::all();
        for filter_enum in filters {
            if let Ok(column) = Column::from_str(filter_enum.get_name().as_str()) {
                condition = condition.add(Self::filter_condition_column(column, filter_enum));
            }
        }
        condition
    }
}

pub struct PaymentQuery;

impl PaymentQuery {
    pub async fn get_payment_by_id(
        db: &DbConn,
        payment_id: Uuid,
    ) -> Result<PaymentData, AppError> {
        let model = PaymentQueryManager::get_by_id_uuid(db, payment_id).await?;
        Ok(model.into())
    }

    pub async fn get_payments<'a>(
        db: &'a DbConn,
        pagination: &Pagination,
        order: &Order,
        filters: &Vec<FilterEnum>,
    ) -> Result<QueryResult<PaymentData>, AppError> {
        let result = PaymentQueryManager::filter(db, pagination, order, filters).await?;
        let mapped_result = QueryResult {
            total_page: result.total_page,
            result: result.result.into_iter().map(|m| m.into()).collect(),
        };
        Ok(mapped_result)
    }
}