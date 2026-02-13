use sea_orm::Set;

use shared_shared_macro_rule::set_if_some;

use features_booking_entities::booking_seat::{ActiveModel, ModelOptionDto};

pub fn assign(mut active_model: ActiveModel, model_option: ModelOptionDto) -> ActiveModel {
    set_if_some!(active_model.id, model_option.id);
    set_if_some!(active_model.booking_id, model_option.booking_id);
    set_if_some!(active_model.seat_id, model_option.seat_id);
    set_if_some!(active_model.price, model_option.price);

    active_model
}
