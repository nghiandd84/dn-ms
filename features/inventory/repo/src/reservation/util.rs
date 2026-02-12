use sea_orm::Set;

use shared_shared_macro_rule::set_if_some;

use features_inventory_entities::reservation::{ActiveModel, ModelOptionDto};

pub fn assign(mut active_model: ActiveModel, model_option: ModelOptionDto) -> ActiveModel {
    set_if_some!(active_model.id, model_option.id);
    set_if_some!(active_model.seat_id, model_option.seat_id);
    set_if_some!(active_model.event_id, model_option.event_id);
    set_if_some!(active_model.user_id, model_option.user_id);
    set_if_some!(active_model.expires_at, model_option.expires_at);
    set_if_some!(active_model.status, model_option.status);
    set_if_some!(active_model.created_at, model_option.created_at);
    set_if_some!(active_model.updated_at, model_option.updated_at);

    active_model
}
