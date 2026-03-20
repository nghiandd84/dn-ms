use sea_orm::Set;

use shared_shared_macro_rule::set_if_some;

use features_merchant_entities::api_key::{ActiveModel, ModelOptionDto};

pub fn assign(mut active_model: ActiveModel, model_option: ModelOptionDto) -> ActiveModel {
    set_if_some!(active_model.id, model_option.id);
    set_if_some!(active_model.environment, model_option.environment);
    set_if_some!(active_model.api_key, model_option.api_key);
    set_if_some!(active_model.merchant_id, model_option.merchant_id);
    set_if_some!(active_model.created_at, model_option.created_at);
    set_if_some!(active_model.updated_at, model_option.updated_at);
    active_model
}
