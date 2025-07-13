use sea_orm::Set;

use shared_shared_macro_rule::set_if_some;

use features_auth_entities::token::{ActiveModel, ModelOptionDto};

pub fn assign(mut active_model: ActiveModel, model_option: ModelOptionDto) -> ActiveModel {
    set_if_some!(active_model.id, model_option.id);
    set_if_some!(active_model.access_token, model_option.access_token);
    set_if_some!(active_model.refresh_token, model_option.refresh_token);
    set_if_some!(active_model.user_id, model_option.user_id);
    set_if_some!(active_model.client_id, model_option.client_id);  
    set_if_some!(active_model.scopes, model_option.scopes);
    set_if_some!(active_model.access_token_expires_at, model_option.access_token_expires_at);
    set_if_some!(active_model.refresh_token_expires_at, model_option.refresh_token_expires_at);
    set_if_some!(active_model.revoked_at, model_option.revoked_at);
    set_if_some!(active_model.created_at, model_option.created_at);
    set_if_some!(active_model.updated_at, model_option.updated_at);

    active_model
}
