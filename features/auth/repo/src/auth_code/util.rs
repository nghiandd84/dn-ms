use sea_orm::Set;

use shared_shared_macro_rule::set_if_some;

use features_auth_entities::auth_code::{ActiveModel, ModelOptionDto};

pub fn assign(mut active_model: ActiveModel, model_option: ModelOptionDto) -> ActiveModel {
    set_if_some!(active_model.id, model_option.id);
    set_if_some!(active_model.user_id, model_option.user_id);
    set_if_some!(active_model.client_id, model_option.client_id);
    set_if_some!(active_model.scopes, model_option.scopes);
    active_model
}
