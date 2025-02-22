use sea_orm::Set;

use shared_shared_macro_rule::set_if_some;

use features_bakery_entities::order::{ActiveModel, ModelOptionDto};

pub fn assign(mut active_model: ActiveModel, model_option: ModelOptionDto) -> ActiveModel {
    set_if_some!(active_model.id, model_option.id);
    set_if_some!(active_model.total, model_option.total);
    set_if_some!(active_model.customer_id, model_option.customer_id);
    set_if_some!(active_model.bakery_id, model_option.bakery_id);
    set_if_some!(active_model.placed_at, model_option.placed_at);

    active_model
}
