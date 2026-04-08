use sea_orm::Set;

use shared_shared_macro_rule::set_if_some;

use features_fee_entities::fee_configuration::{ActiveModel, ModelOptionDto};

pub fn assign(mut active_model: ActiveModel, model_option: ModelOptionDto) -> ActiveModel {
    set_if_some!(active_model.id, model_option.id);
    set_if_some!(active_model.merchant_id, model_option.merchant_id);
    set_if_some!(active_model.pricing_model, model_option.pricing_model);
    set_if_some!(active_model.percentage_rate, model_option.percentage_rate);
    set_if_some!(active_model.fixed_amount, model_option.fixed_amount);
    set_if_some!(active_model.min_fee, model_option.min_fee);
    set_if_some!(active_model.max_fee, model_option.max_fee);
    set_if_some!(active_model.tier_config, model_option.tier_config);
    set_if_some!(active_model.effective_from, model_option.effective_from);
    set_if_some!(active_model.effective_to, model_option.effective_to);
    set_if_some!(active_model.created_at, model_option.created_at);
    set_if_some!(active_model.updated_at, model_option.updated_at);
    active_model
}
