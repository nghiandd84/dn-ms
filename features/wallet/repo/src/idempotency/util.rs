use sea_orm::Set;

use shared_shared_macro_rule::set_if_some;

use features_wallet_entities::idempotency::{ActiveModel, ModelOptionDto};

pub fn assign(mut active_model: ActiveModel, model_option: ModelOptionDto) -> ActiveModel {
    set_if_some!(active_model.id, model_option.id);
    set_if_some!(active_model.key, model_option.key);
    set_if_some!(active_model.endpoint, model_option.endpoint);
    set_if_some!(active_model.request_hash, model_option.request_hash);
    set_if_some!(active_model.response_status, model_option.response_status);
    set_if_some!(active_model.state, model_option.state);
    set_if_some!(active_model.created_at, model_option.created_at);
    set_if_some!(active_model.expires_at, model_option.expires_at);

    active_model
}