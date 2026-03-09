use sea_orm::Set;

use shared_shared_macro_rule::set_if_some;

use features_payments_core_entities::payment_method_limit::{ActiveModel, ModelOptionDto};

pub fn assign(mut active_model: ActiveModel, model_option: ModelOptionDto) -> ActiveModel {
    set_if_some!(active_model.id, model_option.id);
    set_if_some!(active_model.payment_method_id, model_option.payment_method_id);
    set_if_some!(active_model.currency, model_option.currency);
    set_if_some!(active_model.min_amount, model_option.min_amount);
    set_if_some!(active_model.max_amount, model_option.max_amount);
    set_if_some!(active_model.created_at, model_option.created_at);
    set_if_some!(active_model.updated_at, model_option.updated_at);
    active_model
}