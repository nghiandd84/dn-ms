use sea_orm::Set;

use shared_shared_macro_rule::set_if_some;

use features_auth_entities::permission::{ActiveModel, ModelOptionDto};

pub fn assign(mut active_model: ActiveModel, model_option: ModelOptionDto) -> ActiveModel {
    set_if_some!(active_model.id, model_option.id);
    set_if_some!(active_model.resource, model_option.resource);
    set_if_some!(active_model.description, model_option.description);
    set_if_some!(active_model.mask, model_option.mask);

    active_model
}
