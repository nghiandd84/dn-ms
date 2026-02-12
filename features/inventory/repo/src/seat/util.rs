use sea_orm::Set;

use shared_shared_macro_rule::set_if_some;

use features_inventory_entities::seat::{ActiveModel, ModelOptionDto};

pub fn assign(mut active_model: ActiveModel, model_option: ModelOptionDto) -> ActiveModel {
    set_if_some!(active_model.id, model_option.id);
    set_if_some!(active_model.event_id, model_option.event_id);
    set_if_some!(active_model.seat_number, model_option.seat_number);
    set_if_some!(active_model.section, model_option.section);
    set_if_some!(active_model.row_number, model_option.row_number);
    set_if_some!(active_model.seat_type, model_option.seat_type);
    set_if_some!(active_model.price, model_option.price);
    set_if_some!(active_model.status, model_option.status);
    set_if_some!(active_model.created_at, model_option.created_at);
    set_if_some!(active_model.updated_at, model_option.updated_at);
    active_model
}
