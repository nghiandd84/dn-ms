use sea_orm::{ColumnTrait, QueryFilter};
use std::future::Future;
use uuid::Uuid;

use shared_shared_macro::Mutation;

use features_translation_entities::key_tag::{
    ActiveModel, Column, Entity, KeyTagForCreateDto, Model, ModelOptionDto,
};

use crate::key_tag::util::assign;

#[derive(Mutation)]
#[mutation(key_type(Uuid))]
struct KeyTagMutationManager {}

pub struct KeyTagMutation;

impl KeyTagMutation {
    pub fn create_key_tag<'a>(
        data: KeyTagForCreateDto,
    ) -> impl Future<Output = Result<Uuid, DbErr>> + 'a {
        KeyTagMutationManager::create_uuid(data.into())
    }

    pub fn delete_key_tag<'a>(
        key_id: Uuid,
        tag_id: Uuid,
    ) -> impl Future<Output = Result<bool, DbErr>> + 'a {
        async move {
            let result = Entity::delete_many()
                .filter(Column::KeyId.eq(key_id))
                .filter(Column::TagId.eq(tag_id))
                .exec(KeyTagMutationManager::get_db())
                .await?;
            Ok(result.rows_affected > 0)
        }
    }

    pub async fn delete_all_tags_from_key<'a>(
        key_id: Uuid,
    ) -> impl Future<Output = Result<bool, DbErr>> + 'a {
        async move {
            let result = Entity::delete_many()
                .filter(Column::KeyId.eq(key_id))
                .exec(KeyTagMutationManager::get_db())
                .await?;
            Ok(result.rows_affected > 0)
        }
    }
}
