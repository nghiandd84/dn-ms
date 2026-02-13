use sea_orm::Set;

use shared_shared_macro_rule::set_if_some;

use features_booking_entities::booking::{ActiveModel, ModelOptionDto};

pub fn assign(mut active_model: ActiveModel, model_option: ModelOptionDto) -> ActiveModel {
    set_if_some!(active_model.id, model_option.id);
    set_if_some!(active_model.event_id, model_option.event_id);
    set_if_some!(active_model.user_id, model_option.user_id);
    set_if_some!(active_model.total_amount, model_option.total_amount);
    set_if_some!(active_model.status, model_option.status);
    set_if_some!(active_model.payment_id, model_option.payment_id);
    set_if_some!(active_model.payment_status, model_option.payment_status);
    set_if_some!(active_model.booking_reference, model_option.booking_reference);
    set_if_some!(active_model.created_at, model_option.created_at);
    set_if_some!(active_model.confirmed_at, model_option.confirmed_at);

    active_model
}
