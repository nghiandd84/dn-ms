use sea_orm::Set;

use shared_shared_macro_rule::set_if_some;

use features_translation_entities::project::{ActiveModel, ModelOptionDto};

pub fn assign(mut active_model: ActiveModel, model_option: ModelOptionDto) -> ActiveModel {
    set_if_some!(active_model.name, model_option.name);
    set_if_some!(active_model.default_locale, model_option.default_locale);
    set_if_some!(active_model.user_id, model_option.user_id);
    set_if_some!(active_model.api_key, model_option.api_key);
    set_if_some!(active_model.id, model_option.id);
    set_if_some!(active_model.created_at, model_option.created_at);

    active_model
}
