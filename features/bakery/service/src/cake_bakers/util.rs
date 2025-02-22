use sea_orm::Set;

use shared_shared_macro_rule::set_if_some;

use features_bakery_entities::cakes_bakers::{ActiveModel, ModelOptionDto};

pub fn assign(mut active_model: ActiveModel, model_option: ModelOptionDto) -> ActiveModel {
    set_if_some!(active_model.cake_id, model_option.cake_id);
    set_if_some!(active_model.baker_id, model_option.baker_id);

    active_model
}
