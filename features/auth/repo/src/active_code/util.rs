use sea_orm::Set;

use shared_shared_macro_rule::set_if_some;

use features_auth_entities::active_code::{ActiveModel, ModelOptionDto};

pub fn assign(mut active_model: ActiveModel, model_option: ModelOptionDto) -> ActiveModel {
    set_if_some!(active_model.id, model_option.id);
    set_if_some!(active_model.user_id, model_option.user_id);
    set_if_some!(active_model.code, model_option.code);
    set_if_some!(active_model.is_used, model_option.is_used);
    active_model
}
