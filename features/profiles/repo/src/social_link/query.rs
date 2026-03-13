use shared_shared_data_core::{
    filter::{FilterEnum, FilterParam},
    order::Order,
    paging::{Pagination, QueryResult},
};
use shared_shared_data_error::app::AppError;
use shared_shared_macro::Query;
use std::vec;

use features_profiles_entities::social_link::{ActiveModel, Column, Entity, ModelOptionDto};
use features_profiles_model::SocialLinkData;

#[derive(Query)]
#[query(key_type(Uuid))]
#[query_filter(column_name(Column))]
struct SocialLinkQueryManager;

impl SocialLinkQueryManager {
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

pub struct SocialLinkQuery;

impl SocialLinkQuery {
    pub async fn get_social_link_by_id(link_id: Uuid) -> Result<SocialLinkData, AppError> {
        let pagination = Pagination::new(1, 1);
        let order = Order::default();

        let param: FilterParam<Uuid> = FilterParam {
            name: Column::Id.to_string(),
            operator: FilterOperator::Equal,
            value: Some(link_id),
            raw_value: link_id.to_string(),
        };
        let id_filter = FilterEnum::Uuid(param);
        let filters: Vec<FilterEnum> = vec![id_filter];

        let result = SocialLinkQueryManager::filter(&pagination, &order, &filters).await?;
        let dto = result.result.into_iter().next();

        if dto.is_none() {
            return Err(AppError::EntityNotFound {
                entity: "social_link".to_string(),
            });
        }

        Ok(dto.unwrap().into())
    }

    pub async fn get_social_links_by_profile_id(
        profile_id: Uuid,
    ) -> Result<Vec<SocialLinkData>, AppError> {
        let pagination = Pagination::new(1, 100);
        let order = Order::default();

        let param: FilterParam<Uuid> = FilterParam {
            name: Column::ProfileId.to_string(),
            operator: FilterOperator::Equal,
            value: Some(profile_id),
            raw_value: profile_id.to_string(),
        };
        let profile_id_filter = FilterEnum::Uuid(param);
        let filters: Vec<FilterEnum> = vec![profile_id_filter];

        let result = SocialLinkQueryManager::filter(&pagination, &order, &filters).await?;
        let result: Vec<SocialLinkData> = result.result.into_iter().map(|dto| dto.into()).collect();
        Ok(result)
    }

    pub async fn get_social_links(
        pagination: &Pagination,
        order: &Order,
        filters: &Vec<FilterEnum>,
    ) -> Result<QueryResult<SocialLinkData>, AppError> {
        let result = SocialLinkQueryManager::filter(pagination, order, filters).await?;

        let mapped_result = QueryResult {
            total_page: result.total_page,
            result: result.result.into_iter().map(|m| m.into()).collect(),
        };
        Ok(mapped_result)
    }
}
