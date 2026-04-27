use uuid::Uuid;

use shared_shared_data_core::{
    filter::{FilterEnum, FilterParam},
    order::Order,
    paging::{Pagination, QueryResult},
};
use shared_shared_data_error::app::AppError;
use shared_shared_macro::Query;

use features_profiles_entities::user_preference::{ActiveModel, Column, Entity, ModelOptionDto};
use features_profiles_model::UserPreferenceData;

#[derive(Query)]
#[query(key_type(Uuid))]
#[query_filter(column_name(Column))]
struct UserPreferenceQueryManager;



pub struct UserPreferenceQuery;

impl UserPreferenceQuery {
    pub async fn get_user_preference_by_id(
        preference_id: Uuid,
    ) -> Result<UserPreferenceData, AppError> {
        let pagination = Pagination::new(1, 1);
        let order = Order::default();

        let param: FilterParam<Uuid> = FilterParam {
            name: Column::Id.to_string(),
            operator: FilterOperator::Equal,
            value: Some(preference_id),
            raw_value: preference_id.to_string(),
        };
        let id_filter = FilterEnum::Uuid(param);
        let filters: Vec<FilterEnum> = vec![id_filter];

        let result = UserPreferenceQueryManager::filter(&pagination, &order, &filters).await?;
        let dto = result.result.into_iter().next();

        if dto.is_none() {
            return Err(AppError::EntityNotFound {
                entity: "user_preference".to_string(),
            });
        }

        Ok(dto.unwrap().into())
    }

    pub async fn get_user_preference_by_profile_id(
        profile_id: Uuid,
    ) -> Result<UserPreferenceData, AppError> {
        let pagination = Pagination::new(1, 1);
        let order = Order::default();

        let param: FilterParam<Uuid> = FilterParam {
            name: Column::ProfileId.to_string(),
            operator: FilterOperator::Equal,
            value: Some(profile_id),
            raw_value: profile_id.to_string(),
        };
        let profile_id_filter = FilterEnum::Uuid(param);
        let filters: Vec<FilterEnum> = vec![profile_id_filter];

        let result = UserPreferenceQueryManager::filter(&pagination, &order, &filters).await?;
        let dto = result.result.into_iter().next();

        if dto.is_none() {
            return Err(AppError::EntityNotFound {
                entity: "user_preference".to_string(),
            });
        }

        Ok(dto.unwrap().into())
    }

    pub async fn get_user_preferences(
        pagination: &Pagination,
        order: &Order,
        filters: &Vec<FilterEnum>,
    ) -> Result<QueryResult<UserPreferenceData>, AppError> {
        let result = UserPreferenceQueryManager::filter(pagination, order, filters).await?;
        let mapped_result = QueryResult {
            total_page: result.total_page,
            result: result.result.into_iter().map(|m| m.into()).collect(),
        };
        Ok(mapped_result)
    }
}
