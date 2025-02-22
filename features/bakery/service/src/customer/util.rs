use sea_orm::Set;

use shared_shared_macro_rule::set_if_some;

use features_bakery_entities::customer::{ActiveModel, ModelOptionDto};

pub fn assign(mut active_model: ActiveModel, model_option: ModelOptionDto) -> ActiveModel {
    set_if_some!(active_model.id, model_option.id);
    set_if_some!(active_model.name, model_option.name);
    set_if_some!(active_model.notes, model_option.notes);

    active_model
}
