use sea_orm::Set;

use shared_shared_macro_rule::set_if_some;

use features_tagging_entities::entity_tag::{ActiveModel, ModelOptionDto};

pub fn assign(mut active_model: ActiveModel, model_option: ModelOptionDto) -> ActiveModel {
    set_if_some!(active_model.id, model_option.id);
    set_if_some!(active_model.tag_id, model_option.tag_id);
    set_if_some!(active_model.entity_type, model_option.entity_type);
    set_if_some!(active_model.entity_id, model_option.entity_id);
    set_if_some!(active_model.tenant_id, model_option.tenant_id);
    set_if_some!(active_model.tagged_by, model_option.tagged_by);
    set_if_some!(active_model.created_at, model_option.created_at);
    active_model
}
