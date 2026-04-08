use sea_orm::Set;

use shared_shared_macro_rule::set_if_some;

use features_payments_core_entities::payment_method::{ActiveModel, ModelOptionDto};

pub fn assign(mut active_model: ActiveModel, model_option: ModelOptionDto) -> ActiveModel {
    set_if_some!(active_model.id, model_option.id);
    set_if_some!(active_model.display_name, model_option.display_name);
    set_if_some!(active_model.provider_name, model_option.provider_name);
    set_if_some!(active_model.provider_config, model_option.provider_config);
    set_if_some!(
        active_model.supported_countries,
        model_option.supported_countries
    );
    set_if_some!(
        active_model.supported_currencies,
        model_option.supported_currencies
    );
    set_if_some!(active_model.priority, model_option.priority);
    set_if_some!(active_model.is_active, model_option.is_active);
    set_if_some!(active_model.fee_percentage, model_option.fee_percentage);
    set_if_some!(active_model.icon_url, model_option.icon_url);
    set_if_some!(active_model.created_at, model_option.created_at);
    set_if_some!(active_model.updated_at, model_option.updated_at);
    active_model
}
