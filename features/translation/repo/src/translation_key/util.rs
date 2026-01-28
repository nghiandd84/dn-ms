use sea_orm::Set;

use shared_shared_macro_rule::set_if_some;

use features_translation_entities::translation_key::{ActiveModel, ModelOptionDto};

pub fn assign(mut active_model: ActiveModel, model_option: ModelOptionDto) -> ActiveModel {
    set_if_some!(active_model.key_name, model_option.key_name);
    set_if_some!(active_model.description, model_option.description);

    active_model
}
