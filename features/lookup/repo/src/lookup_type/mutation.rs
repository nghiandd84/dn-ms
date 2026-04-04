use shared_shared_macro::Mutation;

use features_lookup_entities::lookup_type::{
    ActiveModel, Column, Entity, LookupTypeForCreateDto, LookupTypeForUpdateDto, Model,
    ModelOptionDto,
};

use crate::lookup_type::util::assign;

#[derive(Mutation)]
#[mutation(key_type(Uuid))]
struct LookupTypeMutationManager {}

pub struct LookupTypeMutation;

impl LookupTypeMutation {
    pub fn create_lookup_type<'a>(
        data: LookupTypeForCreateDto,
    ) -> impl std::future::Future<Output = Result<Uuid, DbErr>> + 'a {
        LookupTypeMutationManager::create_uuid(data.into())
    }

    pub fn update_lookup_type<'a>(
        lookup_type_id: Uuid,
        data: LookupTypeForUpdateDto,
    ) -> impl std::future::Future<Output = Result<bool, DbErr>> + 'a {
        LookupTypeMutationManager::update_by_id_uuid(lookup_type_id, data.into())
    }

    pub fn delete_lookup_type<'a>(
        lookup_type_id: Uuid,
    ) -> impl std::future::Future<Output = Result<bool, DbErr>> + 'a {
        LookupTypeMutationManager::delete_by_id_uuid(lookup_type_id)
    }
}
