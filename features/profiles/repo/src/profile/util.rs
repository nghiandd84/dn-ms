use sea_orm::Set;

use shared_shared_macro_rule::set_if_some;

use features_profiles_entities::profile::{ActiveModel, ModelOptionDto};

pub fn assign(mut active_model: ActiveModel, model_option: ModelOptionDto) -> ActiveModel {
    set_if_some!(active_model.first_name, model_option.first_name);
    set_if_some!(active_model.last_name, model_option.last_name);
    set_if_some!(active_model.bio, model_option.bio);
    set_if_some!(active_model.avatar_url, model_option.avatar_url);
    set_if_some!(active_model.location, model_option.location);

    active_model
}
