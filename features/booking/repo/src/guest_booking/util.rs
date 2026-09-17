use sea_orm::Set;

use shared_shared_macro_rule::set_if_some;

use features_booking_entities::guest_booking::{ActiveModel, ModelOptionDto};

pub fn assign(mut active_model: ActiveModel, model_option: ModelOptionDto) -> ActiveModel {
    set_if_some!(active_model.id, model_option.id);
    set_if_some!(active_model.booking_type, model_option.booking_type);
    set_if_some!(active_model.booking_mode, model_option.booking_mode);
    set_if_some!(active_model.resource_type, model_option.resource_type);
    set_if_some!(active_model.resource_id, model_option.resource_id);
    set_if_some!(active_model.external_ref, model_option.external_ref);
    set_if_some!(active_model.site_origin, model_option.site_origin);
    set_if_some!(active_model.confirm_path, model_option.confirm_path);
    set_if_some!(active_model.guest_email, model_option.guest_email);
    set_if_some!(active_model.guest_name, model_option.guest_name);
    set_if_some!(active_model.total_amount, model_option.total_amount);
    set_if_some!(active_model.currency, model_option.currency);
    set_if_some!(active_model.status, model_option.status);
    set_if_some!(
        active_model.booking_reference,
        model_option.booking_reference
    );
    set_if_some!(
        active_model.confirm_token_hash,
        model_option.confirm_token_hash
    );
    set_if_some!(active_model.expires_at, model_option.expires_at);
    set_if_some!(active_model.metadata, model_option.metadata);
    set_if_some!(
        active_model.promoted_booking_id,
        model_option.promoted_booking_id
    );
    set_if_some!(active_model.created_at, model_option.created_at);
    set_if_some!(active_model.confirmed_at, model_option.confirmed_at);
    set_if_some!(
        active_model.payment_expires_at,
        model_option.payment_expires_at
    );

    active_model
}
