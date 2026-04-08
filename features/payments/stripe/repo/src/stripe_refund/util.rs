use sea_orm::Set;

use shared_shared_macro_rule::set_if_some;

use features_payments_stripe_entities::stripe_refund::{ActiveModel, ModelOptionDto};

pub fn assign(mut active_model: ActiveModel, model_option: ModelOptionDto) -> ActiveModel {
    set_if_some!(active_model.id, model_option.id);
    set_if_some!(active_model.payment_id, model_option.payment_id);
    set_if_some!(active_model.stripe_refund_id, model_option.stripe_refund_id);
    set_if_some!(
        active_model.stripe_payment_intent_id,
        model_option.stripe_payment_intent_id
    );
    set_if_some!(active_model.amount, model_option.amount);
    set_if_some!(active_model.currency, model_option.currency);
    set_if_some!(active_model.status, model_option.status);
    set_if_some!(active_model.reason, model_option.reason);
    set_if_some!(active_model.metadata, model_option.metadata);
    set_if_some!(active_model.created_at, model_option.created_at);
    set_if_some!(active_model.updated_at, model_option.updated_at);
    active_model
}
