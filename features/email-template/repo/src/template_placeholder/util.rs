use sea_orm::Set;

use shared_shared_macro_rule::set_if_some;

use features_email_template_entities::template_placeholders::{ActiveModel, ModelOptionDto};

pub fn assign(mut active_model: ActiveModel, model_option: ModelOptionDto) -> ActiveModel {
    set_if_some!(active_model.id, model_option.id);
    set_if_some!(active_model.template_id, model_option.template_id);
    set_if_some!(active_model.placeholder_key, model_option.placeholder_key);
    set_if_some!(active_model.description, model_option.description);
    set_if_some!(active_model.example_value, model_option.example_value);
    set_if_some!(active_model.is_required, model_option.is_required);

    active_model
}
