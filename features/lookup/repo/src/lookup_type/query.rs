use tracing::debug;
use uuid::Uuid;

use shared_shared_data_core::{
    filter::{FilterEnum, FilterParam},
    order::Order,
    paging::{Pagination, QueryResult},
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
        includes: Vec<String>,
    ) -> Result<LookupTypeData, AppError> {
        debug!(
            "Getting lookup_type by id: {}, includes: {:?}",
            id, includes
        );
        if includes.contains(&"items".to_string()) {
            debug!("Including related items for lookup_type id: {}", id);
            let (lookup_type_model, items) = Entity::find_by_id(id)
                .find_with_related(ItemEntity)
                .all(LookupTypeQueryManager::get_db())
                .await?
                .into_iter()
                .next()
                .ok_or_else(|| DbErr::RecordNotFound("Not found".to_string()))?;

            let mut model: ModelOptionDto = lookup_type_model.into();
            model.items = Some(items.into_iter().map(|item| item.into()).collect());
            debug!("Mapped lookup_type model: {:?}", model);
            Ok(model.into())
        } else {
            debug!("Not including related items for lookup_type id: {}", id);
            let lookup_type_model = LookupTypeQueryManager::get_by_id_uuid(id).await?;
            debug!("Found lookup_type model: {:?}", lookup_type_model);
            Ok(lookup_type_model.into())
        }
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
        includes: Vec<String>,
    ) -> Result<QueryResult<LookupTypeData>, AppError> {
        let mut filters = filters.clone();
        debug!("Getting lookup_types for tenant_id: {}, includes: {:?}, filters: {:?}, pagination: {:?}, order: {:?}", tenant_id, includes, filters, pagination, order);
        if !tenant_id.is_empty() {
            filters.push(FilterEnum::String(FilterParam {
                name: Column::TenantId.to_string(),
                operator: FilterOperator::Equal,
                value: Some(tenant_id.to_string()),
                raw_value: tenant_id.to_string(),
            }));
        }

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
