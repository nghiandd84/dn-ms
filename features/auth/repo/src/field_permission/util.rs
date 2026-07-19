use sea_orm::Set;

use shared_shared_macro_rule::set_if_some;

use features_auth_entities::field_permission::{ActiveModel, ModelOptionDto};

pub fn assign(mut active_model: ActiveModel, model_option: ModelOptionDto) -> ActiveModel {
    set_if_some!(active_model.id, model_option.id);
    set_if_some!(active_model.role_id, model_option.role_id);
    set_if_some!(active_model.resource, model_option.resource);
    set_if_some!(active_model.action, model_option.action);
    set_if_some!(active_model.fields, model_option.fields);

    active_model
}
