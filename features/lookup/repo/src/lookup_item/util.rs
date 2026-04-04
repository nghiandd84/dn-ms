use sea_orm::Set;

use shared_shared_macro_rule::set_if_some;

use features_lookup_entities::lookup_item::{ActiveModel, ModelOptionDto};

pub fn assign(mut active_model: ActiveModel, model_option: ModelOptionDto) -> ActiveModel {
    set_if_some!(active_model.id, model_option.id);
    set_if_some!(active_model.lookup_type_id, model_option.lookup_type_id);
    set_if_some!(active_model.code, model_option.code);
    set_if_some!(active_model.name, model_option.name);
    set_if_some!(active_model.url, model_option.url);
    set_if_some!(active_model.query_param_one, model_option.query_param_one);
    set_if_some!(active_model.query_param_two, model_option.query_param_two);
    set_if_some!(active_model.tenants, model_option.tenants);
    set_if_some!(active_model.is_active, model_option.is_active);
    set_if_some!(active_model.sort_order, model_option.sort_order);
    set_if_some!(active_model.created_at, model_option.created_at);
    set_if_some!(active_model.updated_at, model_option.updated_at);
    active_model
}
