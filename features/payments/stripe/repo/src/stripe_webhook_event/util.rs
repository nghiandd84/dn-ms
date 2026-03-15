use sea_orm::Set;

use shared_shared_macro_rule::set_if_some;

use features_payments_stripe_entities::stripe_webhook_event::{ActiveModel, ModelOptionDto};

pub fn assign(mut active_model: ActiveModel, model_option: ModelOptionDto) -> ActiveModel {
    set_if_some!(active_model.id, model_option.id);
    set_if_some!(active_model.stripe_event_id, model_option.stripe_event_id);
    set_if_some!(active_model.event_type, model_option.event_type);
    set_if_some!(active_model.event_data, model_option.event_data);
    set_if_some!(active_model.processed, model_option.processed);
    set_if_some!(active_model.processing_error, model_option.processing_error);
    set_if_some!(active_model.created_at, model_option.created_at);
    set_if_some!(active_model.updated_at, model_option.updated_at);
    active_model
}
