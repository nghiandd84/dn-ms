use sea_orm::Set;

use shared_shared_macro_rule::set_if_some;

use features_profiles_entities::user_preference::{ActiveModel, ModelOptionDto};

pub fn assign(mut active_model: ActiveModel, model_option: ModelOptionDto) -> ActiveModel {
    set_if_some!(active_model.language, model_option.language);
    set_if_some!(active_model.theme, model_option.theme);
    set_if_some!(active_model.notifications_enabled, model_option.notifications_enabled);

    active_model
}
