use sea_orm::Set;
use tracing::warn;

use shared_shared_macro_rule::set_if_some;

use features_auth_entities::user::{ActiveModel, ModelOptionDto};

pub fn assign(mut active_model: ActiveModel, model_option: ModelOptionDto) -> ActiveModel {
    set_if_some!(active_model.id, model_option.id);
    set_if_some!(active_model.email, model_option.email);
    if let Some(password) = model_option.password {
        warn!("Password is changing");
        active_model.password = Set(password);
    }

    active_model
}

