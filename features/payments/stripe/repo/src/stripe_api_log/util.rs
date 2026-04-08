use sea_orm::Set;

use shared_shared_macro_rule::set_if_some;

use features_payments_stripe_entities::stripe_api_log::{ActiveModel, ModelOptionDto};

pub fn assign(mut active_model: ActiveModel, model_option: ModelOptionDto) -> ActiveModel {
    set_if_some!(active_model.id, model_option.id);
    set_if_some!(active_model.endpoint, model_option.endpoint);
    set_if_some!(active_model.method, model_option.method);
    set_if_some!(active_model.request_body, model_option.request_body);
    set_if_some!(active_model.response_body, model_option.response_body);
    set_if_some!(active_model.status_code, model_option.status_code);
    set_if_some!(active_model.error_message, model_option.error_message);
    set_if_some!(
        active_model.stripe_request_id,
        model_option.stripe_request_id
    );
    set_if_some!(active_model.created_at, model_option.created_at);
    set_if_some!(active_model.updated_at, model_option.updated_at);
    active_model
}
