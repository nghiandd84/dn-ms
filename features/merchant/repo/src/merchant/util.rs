use sea_orm::Set;

use shared_shared_macro_rule::set_if_some;

use features_merchant_entities::merchant::{ActiveModel, ModelOptionDto};

pub fn assign(mut active_model: ActiveModel, model_option: ModelOptionDto) -> ActiveModel {
    set_if_some!(active_model.id, model_option.id);
    set_if_some!(active_model.business_name, model_option.business_name);
    set_if_some!(active_model.email, model_option.email);
    set_if_some!(active_model.phone, model_option.phone);
    set_if_some!(active_model.business_type, model_option.business_type);
    set_if_some!(active_model.kyc_status, model_option.kyc_status);
    set_if_some!(active_model.kyc_verified_at, model_option.kyc_verified_at);
    set_if_some!(active_model.status, model_option.status);
    set_if_some!(active_model.created_at, model_option.created_at);
    set_if_some!(active_model.updated_at, model_option.updated_at);
    active_model
}