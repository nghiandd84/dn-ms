use sea_orm::Set;

use shared_shared_macro_rule::set_if_some;

use features_email_template_entities::template_translations::{ActiveModel, ModelOptionDto};

pub fn assign(mut active_model: ActiveModel, model_option: ModelOptionDto) -> ActiveModel {
    set_if_some!(active_model.id, model_option.id);
    set_if_some!(active_model.language_code, model_option.language_code);
    set_if_some!(active_model.subject, model_option.subject);
    set_if_some!(active_model.body, model_option.body);
    set_if_some!(active_model.version_name, model_option.version_name);

    active_model
}
