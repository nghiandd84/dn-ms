use sea_orm::{DbConn, DbErr};

use uuid::Uuid;

use crate::{
    filter::FilterEnum,
    order::Order,
    paging::{Pagination, QueryResult},
};

pub trait QueryManager<AM, MD> {
    fn get_by_id_uuid(db: &DbConn, id: Uuid) -> impl std::future::Future<Output = Result<MD, DbErr>>;

    fn get_by_id_i32(db: &DbConn, id: i32) -> impl std::future::Future<Output = Result<MD, DbErr>>;

    fn filter(
        db: &DbConn,
        pagination: &Pagination,
        order: &Order,
        filter: &Vec<FilterEnum>,
    ) -> impl std::future::Future<Output = Result<QueryResult<MD>, DbErr>>;
}
