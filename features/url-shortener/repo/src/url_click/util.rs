use sea_orm::Set;

use shared_shared_macro_rule::set_if_some;

use features_url_shortener_entities::url_click::{ActiveModel, ModelOptionDto};

pub fn assign(mut active_model: ActiveModel, model_option: ModelOptionDto) -> ActiveModel {
    set_if_some!(active_model.id, model_option.id);
    set_if_some!(active_model.url_id, model_option.url_id);
    set_if_some!(active_model.ip_address, model_option.ip_address);
    set_if_some!(active_model.user_agent, model_option.user_agent);
    set_if_some!(active_model.referrer, model_option.referrer);
    set_if_some!(active_model.country, model_option.country);
    set_if_some!(active_model.clicked_at, model_option.clicked_at);
    set_if_some!(active_model.created_at, model_option.created_at);
    active_model
}
