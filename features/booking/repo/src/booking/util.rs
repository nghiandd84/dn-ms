use sea_orm::Set;

use shared_shared_macro_rule::set_if_some;

use features_booking_entities::booking::{ActiveModel, ModelOptionDto};

pub fn assign(mut active_model: ActiveModel, model_option: ModelOptionDto) -> ActiveModel {
    set_if_some!(active_model.id, model_option.id);
    set_if_some!(active_model.booking_type, model_option.booking_type);
    set_if_some!(active_model.booking_mode, model_option.booking_mode);
    set_if_some!(active_model.resource_type, model_option.resource_type);
    set_if_some!(active_model.resource_id, model_option.resource_id);
    set_if_some!(active_model.external_ref, model_option.external_ref);
    set_if_some!(active_model.user_id, model_option.user_id);
    set_if_some!(active_model.total_amount, model_option.total_amount);
    set_if_some!(active_model.currency, model_option.currency);
    set_if_some!(active_model.status, model_option.status);
    set_if_some!(active_model.payment_id, model_option.payment_id);
    set_if_some!(active_model.payment_status, model_option.payment_status);
    set_if_some!(
        active_model.booking_reference,
        model_option.booking_reference
    );
    set_if_some!(active_model.metadata, model_option.metadata);
    set_if_some!(active_model.version, model_option.version);
    set_if_some!(active_model.created_at, model_option.created_at);
    set_if_some!(active_model.confirmed_at, model_option.confirmed_at);

    active_model
}
