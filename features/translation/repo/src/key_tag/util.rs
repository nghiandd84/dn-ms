use sea_orm::Set;

use shared_shared_macro_rule::set_if_some;

use features_translation_entities::key_tag::{ActiveModel, ModelOptionDto};

pub fn assign(mut active_model: ActiveModel, model_option: ModelOptionDto) -> ActiveModel {
    set_if_some!(active_model.key_id, model_option.key_id);
    set_if_some!(active_model.tag_id, model_option.tag_id);
    set_if_some!(active_model.id, model_option.id);
    set_if_some!(active_model.created_at, model_option.created_at);

    active_model
}
