use uuid::Uuid;
use sea_orm::{DbConn, DbErr};
pub trait MutationManager<AM, M, MD>
where
    AM: From<M>,
{
    fn create_uuid(db: &DbConn, model: M) -> impl std::future::Future<Output = Result<Uuid, DbErr>>;
    fn bulk_create_uuid(db: &DbConn, models: Vec<M>) -> impl std::future::Future<Output = Result<Vec<Uuid>, DbErr>>;
    fn create_i32(db: &DbConn, model: M) -> impl std::future::Future<Output = Result<i32, DbErr>>;
    fn bulk_create_i32(db: &DbConn, models: Vec<M>) -> impl std::future::Future<Output = Result<Vec<i32>, DbErr>>;

    fn update_by_id_uuid(
        db: &DbConn,
        id: Uuid,
        model_option: MD,
    ) -> impl std::future::Future<Output = Result<bool, DbErr>>;

    fn bulk_update_by_id_uuid(
        db: &DbConn,
        data: Vec<(Uuid, MD)>,
    ) -> impl std::future::Future<Output = Result<Vec<Uuid>, DbErr>>;

    fn update_by_id_i32(
        db: &DbConn,
        id: i32,
        model_option: MD,
    ) -> impl std::future::Future<Output = Result<bool, DbErr>>;

    fn bulk_update_by_id_i32(
        db: &DbConn,
        data: Vec<(i32, MD)>,
    ) -> impl std::future::Future<Output = Result<Vec<i32>, DbErr>>;
    
    fn delete_by_id_uuid(
        db: &DbConn,
        id: Uuid,
    ) -> impl std::future::Future<Output = Result<bool, DbErr>>;

    fn delete_by_id_i32(
        db: &DbConn,
        id: i32,
    ) -> impl std::future::Future<Output = Result<bool, DbErr>>;
    
}
