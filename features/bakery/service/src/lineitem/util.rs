use sea_orm::Set;

use shared_shared_macro_rule::set_if_some;

use features_bakery_entities::lineitem::{ActiveModel, ModelOptionDto};

pub fn assign(mut active_model: ActiveModel, model_option: ModelOptionDto) -> ActiveModel {
    set_if_some!(active_model.price, model_option.price);
    set_if_some!(active_model.quantity, model_option.quantity);
    set_if_some!(active_model.order_id, model_option.order_id);
    set_if_some!(active_model.cake_id, model_option.cake_id);
    set_if_some!(active_model.id, model_option.id);

    active_model
}
