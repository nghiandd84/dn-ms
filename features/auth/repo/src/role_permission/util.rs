use sea_orm::Set;

use shared_shared_macro_rule::set_if_some;

use features_auth_entities::role_permission::{ActiveModel, ModelOptionDto};

pub fn assign(mut active_model: ActiveModel, model_option: ModelOptionDto) -> ActiveModel {
    set_if_some!(active_model.id, model_option.id);
    set_if_some!(active_model.role_id, model_option.role_id);
    set_if_some!(active_model.permission_id, model_option.permission_id);

    active_model
}
