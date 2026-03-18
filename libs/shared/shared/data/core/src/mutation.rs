use sea_orm::DbErr;
use uuid::Uuid;
pub trait MutationManager<AM, M, MD>
where
    AM: From<M>,
{
    fn create_uuid(model: M) -> impl std::future::Future<Output = Result<Uuid, DbErr>>;
    fn create_i32(model: M) -> impl std::future::Future<Output = Result<i32, DbErr>>;
    fn create_str(model: M) -> impl std::future::Future<Output = Result<String, DbErr>>;

    fn bulk_create_uuid(
        models: Vec<M>,
    ) -> impl std::future::Future<Output = Result<Vec<Uuid>, DbErr>>;
    fn bulk_create_i32(
        models: Vec<M>,
    ) -> impl std::future::Future<Output = Result<Vec<i32>, DbErr>>;
    // fn bulk_create_str(
    //     models: Vec<M>,
    // ) -> impl std::future::Future<Output = Result<Vec<String>, DbErr>>;

    fn bulk_update_by_id_uuid(
        data: Vec<(Uuid, MD)>,
    ) -> impl std::future::Future<Output = Result<Vec<Uuid>, DbErr>>;
    fn bulk_update_by_id_i32(
        data: Vec<(i32, MD)>,
    ) -> impl std::future::Future<Output = Result<Vec<i32>, DbErr>>;
    // fn bulk_update_by_id_str(
    //     data: Vec<(String, MD)>,
    // ) -> impl std::future::Future<Output = Result<Vec<String>, DbErr>>;

    fn update_by_id_uuid(
        id: Uuid,
        model_option: MD,
    ) -> impl std::future::Future<Output = Result<bool, DbErr>>;
    fn update_by_id_i32(
        id: i32,
        model_option: MD,
    ) -> impl std::future::Future<Output = Result<bool, DbErr>>;
    // fn update_by_id_str(
    //     id: String,
    //     model_option: MD,
    // ) -> impl std::future::Future<Output = Result<bool, DbErr>>;

    fn delete_by_id_uuid(id: Uuid) -> impl std::future::Future<Output = Result<bool, DbErr>>;
    fn delete_by_id_i32(id: i32) -> impl std::future::Future<Output = Result<bool, DbErr>>;
    // fn delete_by_id_str(id: String) -> impl std::future::Future<Output = Result<bool, DbErr>>;
}
