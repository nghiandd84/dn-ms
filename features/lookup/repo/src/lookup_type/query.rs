use tracing::debug;
use uuid::Uuid;

use shared_shared_data_core::{
    filter::{FilterEnum, FilterParam},
    order::Order,
    paging::{Pagination, QueryResult},
    query_params::QueryParams,
};
use shared_shared_data_error::app::AppError;
use shared_shared_macro::Query;

use features_lookup_entities::lookup_item::Entity as ItemEntity;
use features_lookup_entities::lookup_type::{ActiveModel, Column, Entity, ModelOptionDto};
use features_lookup_model::lookup_type::LookupTypeData;

#[derive(Query)]
#[query(key_type(Uuid))]
#[query_filter(column_name(Column))]
#[query_related(entity(ItemEntity), field(items), name("items"))]
struct LookupTypeQueryManager;

impl LookupTypeQueryManager {
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

pub struct LookupTypeQuery;

impl LookupTypeQuery {
    pub async fn get_lookup_type_by_id(
        id: Uuid,
        query_params: &QueryParams,
    ) -> Result<LookupTypeData, AppError> {
        let includes = query_params.includes();
        debug!(
            "Getting lookup_type by id: {}, includes: {:?}",
            id, includes
        );
        let model = LookupTypeQueryManager::get_by_id_uuid_with_related_entities(id, &includes).await?;
        Ok(model.into())
    }

    pub async fn get_lookup_type_by_code(
        tenant_id: &str,
        code: &str,
    ) -> Result<LookupTypeData, AppError> {
        let code_param: FilterParam<String> = FilterParam {
            name: Column::Code.to_string(),
            operator: FilterOperator::Equal,
            value: Some(code.to_string()),
            raw_value: code.to_string(),
        };
        let mut filters = vec![FilterEnum::String(code_param)];

        if !tenant_id.is_empty() {
            filters.push(FilterEnum::String(FilterParam {
                name: Column::TenantId.to_string(),
                operator: FilterOperator::Equal,
                value: Some(tenant_id.to_string()),
                raw_value: tenant_id.to_string(),
            }));
        }

        let pagination = Pagination::default();
        let order = Order::default();

        let result = LookupTypeQueryManager::filter(&pagination, &order, &filters).await?;
        let item = result
            .result
            .into_iter()
            .next()
            .ok_or(AppError::EntityNotFound {
                entity: format!("lookup_type code {}", code),
            })?;
        Ok(item.into())
    }

    pub async fn get_lookup_types(
        tenant_id: &str,
        pagination: &Pagination,
        order: &Order,
        filters: &Vec<FilterEnum>,
        query_params: &QueryParams,
    ) -> Result<QueryResult<LookupTypeData>, AppError> {
        let mut filters = filters.clone();
        if !tenant_id.is_empty() {
            filters.push(FilterEnum::String(FilterParam {
                name: Column::TenantId.to_string(),
                operator: FilterOperator::Equal,
                value: Some(tenant_id.to_string()),
                raw_value: tenant_id.to_string(),
            }));
        }

        let includes = query_params.includes();
        let result = if !includes.is_empty() {
            LookupTypeQueryManager::filter_with_related_entities(pagination, order, &filters, &includes).await?
        } else {
            LookupTypeQueryManager::filter(pagination, order, &filters).await?
        };
        let mapped_result = QueryResult {
            total_page: result.total_page,
            result: result.result.into_iter().map(|m| m.into()).collect(),
        };
        Ok(mapped_result)
    }
}
