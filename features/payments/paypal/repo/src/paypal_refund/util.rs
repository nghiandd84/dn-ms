use sea_orm::Set;

use shared_shared_macro_rule::set_if_some;

use features_payments_paypal_entities::paypal_refund::{ActiveModel, ModelOptionDto};

pub fn assign(mut active_model: ActiveModel, model_option: ModelOptionDto) -> ActiveModel {
    set_if_some!(active_model.id, model_option.id);
    set_if_some!(active_model.payment_id, model_option.payment_id);
    set_if_some!(active_model.paypal_refund_id, model_option.paypal_refund_id);
    set_if_some!(active_model.paypal_capture_id, model_option.paypal_capture_id);
    set_if_some!(active_model.amount, model_option.amount);
    set_if_some!(active_model.currency, model_option.currency);
    set_if_some!(active_model.status, model_option.status);
    set_if_some!(active_model.reason, model_option.reason);
    set_if_some!(active_model.metadata, model_option.metadata);
    set_if_some!(active_model.created_at, model_option.created_at);
    set_if_some!(active_model.updated_at, model_option.updated_at);
    active_model
}
