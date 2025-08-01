use sea_orm::Set;

use shared_shared_macro_rule::set_if_some;

use features_auth_entities::client::{ActiveModel, ModelOptionDto};

pub fn assign(mut active_model: ActiveModel, model_option: ModelOptionDto) -> ActiveModel {
    set_if_some!(active_model.id, model_option.id);
    set_if_some!(active_model.name, model_option.name);
    set_if_some!(active_model.description, model_option.description);
    set_if_some!(active_model.client_secret, model_option.client_secret);
    set_if_some!(active_model.allowed_grants, model_option.allowed_grants);
    set_if_some!(active_model.redirect_uris, model_option.redirect_uris);

    active_model
}
