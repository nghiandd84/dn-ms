use sea_orm::{entity::ActiveModelTrait, EntityTrait};
use sea_orm::{DbConn, DbErr};

use features_bakery_entities::cakes_bakers::{ActiveModel, CakeBakerForCreateDto, Entity, Model};
use shared_shared_config::db::DB_WRITE;

struct CakeMutationManager {}

impl CakeMutationManager {
    async fn insert(model: Model) -> Result<bool, DbErr> {
        let db: &DbConn = DB_WRITE.get().expect("DB_WRITE is not initialized");
        let active_model: ActiveModel = model.into();
        let _result = active_model.insert(db).await;
        Ok(true)
    }

    async fn delete_by_id(cake_id: i32, baker_id: i32) -> Result<bool, DbErr> {
        let db: &DbConn = DB_WRITE.get().expect("DB_WRITE is not initialized");
        let model: ActiveModel = Entity::find_by_id((cake_id, baker_id))
            .one(db)
            .await?
            .ok_or(DbErr::RecordNotFound("Not found".to_string()))
            .map(Into::into)?;

        model.delete(db).await?;

        Ok(true)
    }
}

pub struct CakeBakerMutation {}

impl CakeBakerMutation {
    pub fn create<'a>(
        data: CakeBakerForCreateDto,
    ) -> impl std::future::Future<Output = Result<bool, DbErr>> + 'a {
        CakeMutationManager::insert(data.into())
    }

    pub fn delete<'a>(
        cake_id: i32,
        baker_id: i32,
    ) -> impl std::future::Future<Output = Result<bool, DbErr>> + 'a {
        CakeMutationManager::delete_by_id(cake_id, baker_id)
    }
}
