use sea_orm::DbErr;

use uuid::Uuid;

use crate::{
    filter::{FilterCondition, FilterEnum},
    order::Order,
    paging::{Pagination, QueryResult},
};

pub trait QueryManager<AM, MD> {
    fn get_by_id_uuid(id: Uuid) -> impl std::future::Future<Output = Result<MD, DbErr>>;

    fn get_by_id_i32(id: i32) -> impl std::future::Future<Output = Result<MD, DbErr>>;
    fn get_by_id_str(id: String) -> impl std::future::Future<Output = Result<MD, DbErr>>;

    fn filter(
        pagination: &Pagination,
        order: &Order,
        filter: &FilterCondition,
    ) -> impl std::future::Future<Output = Result<QueryResult<MD>, DbErr>>;

    fn filter_with_related_entities(
        pagination: &Pagination,
        order: &Order,
        filter: &FilterCondition,
        includes: &Vec<String>,
        related_filters: &Vec<FilterEnum>,
    ) -> impl std::future::Future<Output = Result<QueryResult<MD>, DbErr>> {
        let _ = includes;
        let _ = related_filters;
        Self::filter(pagination, order, filter)
    }

    fn get_by_id_uuid_with_related_entities(
        id: Uuid,
        includes: &Vec<String>,
        related_filters: &Vec<FilterEnum>,
    ) -> impl std::future::Future<Output = Result<MD, DbErr>> {
        let _ = includes;
        let _ = related_filters;
        Self::get_by_id_uuid(id)
    }

    fn get_by_id_i32_with_related_entities(
        id: i32,
        includes: &Vec<String>,
        related_filters: &Vec<FilterEnum>,
    ) -> impl std::future::Future<Output = Result<MD, DbErr>> {
        let _ = includes;
        let _ = related_filters;
        Self::get_by_id_i32(id)
    }

    fn get_by_id_str_with_related_entities(
        id: String,
        includes: &Vec<String>,
        related_filters: &Vec<FilterEnum>,
    ) -> impl std::future::Future<Output = Result<MD, DbErr>> {
        let _ = includes;
        let _ = related_filters;
        Self::get_by_id_str(id)
    }
}
