use sea_orm::Set;

use shared_shared_macro_rule::set_if_some;

use features_auth_entities::authentication::{ActiveModel, ModelOptionDto};

pub fn assign(mut active_model: ActiveModel, model_option: ModelOptionDto) -> ActiveModel {
    set_if_some!(active_model.id, model_option.id);
    set_if_some!(active_model.client_id, model_option.client_id);
    set_if_some!(active_model.scopes, model_option.scopes);
    set_if_some!(active_model.response_type, model_option.response_type);
    set_if_some!(active_model.state, model_option.state);
    set_if_some!(active_model.redirect_uri, model_option.redirect_uri);

    active_model
}
