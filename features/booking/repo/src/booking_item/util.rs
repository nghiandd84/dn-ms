use sea_orm::Set;

use shared_shared_macro_rule::set_if_some;

use features_booking_entities::booking_item::{ActiveModel, ModelOptionDto};

pub fn assign(mut active_model: ActiveModel, model_option: ModelOptionDto) -> ActiveModel {
    set_if_some!(active_model.id, model_option.id);
    set_if_some!(active_model.booking_id, model_option.booking_id);
    set_if_some!(active_model.item_type, model_option.item_type);
    set_if_some!(active_model.item_id, model_option.item_id);
    set_if_some!(active_model.price, model_option.price);
    set_if_some!(active_model.metadata, model_option.metadata);

    active_model
}
