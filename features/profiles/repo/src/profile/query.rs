use uuid::Uuid;

use shared_shared_data_core::{
    filter::{FilterEnum, FilterParam},
    order::Order,
    paging::{Pagination, QueryResult},
};
use shared_shared_data_error::app::AppError;
use shared_shared_macro::Query;

use features_profiles_entities::profile::{ActiveModel, Column, Entity, ModelOptionDto};
use features_profiles_model::ProfileData;

#[derive(Query)]
#[query(key_type(Uuid))]
#[query_filter(column_name(Column))]
struct ProfileQueryManager;

pub struct ProfileQuery;

impl ProfileQuery {
    pub async fn get_profile_by_id(profile_id: Uuid) -> Result<ProfileData, AppError> {
        let model = ProfileQueryManager::get_by_id_uuid(profile_id).await?;
        Ok(model.into())
    }

    pub async fn get_profile_by_user_id(user_id: Uuid) -> Result<ProfileData, AppError> {
        let pagination = Pagination::new(1, 1);
        let order = Order::default();

        let param: FilterParam<Uuid> = FilterParam {
            name: Column::UserId.to_string(),
            operator: FilterOperator::Equal,
            value: Some(user_id),
            raw_value: user_id.to_string(),
        };
        let user_id_filter = FilterEnum::Uuid(param);
        let filters: Vec<FilterEnum> = vec![user_id_filter];

        let result =
            ProfileQueryManager::filter(&pagination, &order, &FilterCondition::from(&filters))
                .await?;
        let dto = result.result.into_iter().next();

        if dto.is_none() {
            return Err(AppError::EntityNotFound {
                entity: "profile".to_string(),
            });
        }

        Ok(dto.unwrap().into())
    }

    pub async fn get_profiles(
        pagination: &Pagination,
        order: &Order,
        filters: &FilterCondition,
    ) -> Result<QueryResult<ProfileData>, AppError> {
        let result = ProfileQueryManager::filter(pagination, order, filters).await?;
        let mapped_result = QueryResult {
            total_page: result.total_page,
            result: result.result.into_iter().map(|m| m.into()).collect(),
        };
        Ok(mapped_result)
    }
}
