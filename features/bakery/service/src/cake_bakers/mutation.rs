use sea_orm::{entity::ActiveModelTrait, EntityTrait};
use sea_orm::{DbConn, DbErr};

use features_bakery_entities::cakes_bakers::{ActiveModel, CakeBakerForCreateDto, Entity, Model};

struct CakeMutationManager {}

impl CakeMutationManager {
    async fn insert(db: &DbConn, model: Model) -> Result<bool, DbErr> {
        let active_model: ActiveModel = model.into();
        let _result = active_model.insert(db).await;
        Ok(true)
    }

    async fn delete_by_id(db: &DbConn, cake_id: i32, baker_id: i32) -> Result<bool, DbErr> {
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
        db: &'a DbConn,
        data: CakeBakerForCreateDto,
    ) -> impl std::future::Future<Output = Result<bool, DbErr>> + 'a {
        CakeMutationManager::insert(db, data.into())
    }

    pub fn delete<'a>(
        db: &'a DbConn,
        cake_id: i32,
        baker_id: i32,
    ) -> impl std::future::Future<Output = Result<bool, DbErr>> + 'a {
        CakeMutationManager::delete_by_id(db, cake_id, baker_id)
    }
}
