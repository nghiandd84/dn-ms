use sea_orm::Set;

use shared_shared_macro_rule::set_if_some;

use features_payments_core_entities::payment::{ActiveModel, ModelOptionDto};

pub fn assign(mut active_model: ActiveModel, model_option: ModelOptionDto) -> ActiveModel {
    set_if_some!(active_model.id, model_option.id);
    set_if_some!(active_model.transaction_id, model_option.transaction_id);
    set_if_some!(active_model.user_id, model_option.user_id);
    set_if_some!(active_model.amount, model_option.amount);
    set_if_some!(active_model.currency, model_option.currency);
    set_if_some!(active_model.status, model_option.status);
    set_if_some!(active_model.provider_name, model_option.provider_name);
    set_if_some!(
        active_model.gateway_transaction_id,
        model_option.gateway_transaction_id
    );
    set_if_some!(active_model.idempotency_key, model_option.idempotency_key);
    set_if_some!(active_model.created_at, model_option.created_at);
    set_if_some!(active_model.updated_at, model_option.updated_at);
    active_model
}
