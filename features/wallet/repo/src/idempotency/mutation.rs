use shared_shared_macro::Mutation;

use features_wallet_entities::idempotency::{
    ActiveModel, Column, Entity, IdempotencyKeyForCreateDto, IdempotencyKeyForUpdateDto, Model,
    ModelOptionDto,
};

use crate::idempotency::util::assign;

#[derive(Mutation)]
#[mutation(key_type(Uuid))]
struct IdempotencyMutationManager {}

pub struct IdempotencyMutation;

impl IdempotencyMutation {
    pub fn create_idempotency_key<'a>(
        data: IdempotencyKeyForCreateDto,
    ) -> impl std::future::Future<Output = Result<Uuid, DbErr>> + 'a {
        IdempotencyMutationManager::create_uuid(data.into())
    }

    pub fn update_idempotency_key<'a>(
        id: Uuid,
        data: IdempotencyKeyForUpdateDto,
    ) -> impl std::future::Future<Output = Result<bool, DbErr>> + 'a {
        IdempotencyMutationManager::update_by_id_uuid(id, data.into())
    }

    pub fn delete_idempotency_key<'a>(
        id: Uuid,
    ) -> impl std::future::Future<Output = Result<bool, DbErr>> + 'a {
        IdempotencyMutationManager::delete_by_id_uuid(id)
    }

    pub async fn create_if_not_exists(
        key: &str,
        endpoint: &str,
        state: &str,
        request_hash: Option<&str>,
        ttl_hours: i64,
    ) -> Result<bool, DbErr> {
        // TOTO: This would need access to the database connection

        // This would need access to the database connection
        // For now, return a placeholder implementation
        // In real implementation, you'd inject the db connection
        Err(DbErr::Custom(
            "Not implemented - needs database connection".to_string(),
        ))
    }

    pub async fn update_completed(
        key: &str,
        status: i32,
        response_body: &str,
    ) -> Result<bool, DbErr> {
        // This would need access to the database connection
        // For now, return a placeholder implementation
        Err(DbErr::Custom(
            "Not implemented - needs database connection".to_string(),
        ))
    }
}
