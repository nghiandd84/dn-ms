use sea_orm::Set;

use shared_shared_macro_rule::set_if_some;

use features_event_entities::event::{ActiveModel, ModelOptionDto};

pub fn assign(mut active_model: ActiveModel, model_option: ModelOptionDto) -> ActiveModel {
    set_if_some!(active_model.id, model_option.id);
    set_if_some!(active_model.event_name, model_option.event_name);
    set_if_some!(active_model.event_date, model_option.event_date);
    set_if_some!(active_model.venue_name, model_option.venue_name);
    set_if_some!(active_model.total_seats, model_option.total_seats);
    set_if_some!(active_model.status, model_option.status);
    set_if_some!(active_model.sale_start_time, model_option.sale_start_time);

    active_model
}
