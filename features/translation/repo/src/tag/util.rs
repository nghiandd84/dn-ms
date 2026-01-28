use sea_orm::Set;

use shared_shared_macro_rule::set_if_some;

use features_translation_entities::tag::{ActiveModel, ModelOptionDto};

pub fn assign(mut active_model: ActiveModel, model_option: ModelOptionDto) -> ActiveModel {
    set_if_some!(active_model.name, model_option.name);

    active_model
}
