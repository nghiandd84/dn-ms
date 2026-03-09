use sea_orm::Set;

use shared_shared_macro_rule::set_if_some;

use features_payments_core_entities::payment_attempt::{ActiveModel, ModelOptionDto};

pub fn assign(mut active_model: ActiveModel, model_option: ModelOptionDto) -> ActiveModel {
    set_if_some!(active_model.id, model_option.id);
    set_if_some!(active_model.payment_id, model_option.payment_id);
    set_if_some!(active_model.provider, model_option.provider);
    set_if_some!(active_model.raw_request, model_option.raw_request);
    set_if_some!(active_model.raw_response, model_option.raw_response);
    set_if_some!(active_model.success, model_option.success);
    set_if_some!(active_model.error_message, model_option.error_message);
    set_if_some!(active_model.created_at, model_option.created_at);
    active_model
}