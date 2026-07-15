use sea_orm::Set;

use shared_shared_macro_rule::set_if_some;

use features_url_shortener_entities::shortened_url::{ActiveModel, ModelOptionDto};

pub fn assign(mut active_model: ActiveModel, model_option: ModelOptionDto) -> ActiveModel {
    set_if_some!(active_model.id, model_option.id);
    set_if_some!(active_model.user_id, model_option.user_id);
    set_if_some!(active_model.original_url, model_option.original_url);
    set_if_some!(active_model.short_code, model_option.short_code);
    set_if_some!(active_model.title, model_option.title);
    set_if_some!(active_model.is_active, model_option.is_active);
    set_if_some!(active_model.click_count, model_option.click_count);
    set_if_some!(active_model.created_at, model_option.created_at);
    set_if_some!(active_model.updated_at, model_option.updated_at);
    active_model
}
