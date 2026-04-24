use sea_orm::Set;

use shared_shared_macro_rule::set_if_some;

use features_lookup_entities::lookup_type::{ActiveModel, ModelOptionDto};

pub fn assign(mut active_model: ActiveModel, model_option: ModelOptionDto) -> ActiveModel {
    set_if_some!(active_model.id, model_option.id);
    set_if_some!(active_model.tenant_id, model_option.tenant_id);
    set_if_some!(active_model.code, model_option.code);
    set_if_some!(active_model.name, model_option.name);
    set_if_some!(active_model.description, model_option.description);
    set_if_some!(active_model.is_active, model_option.is_active);
    set_if_some!(active_model.created_at, model_option.created_at);
    set_if_some!(active_model.updated_at, model_option.updated_at);
    active_model
}
