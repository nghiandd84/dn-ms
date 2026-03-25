use sea_orm::Set;

use shared_shared_macro_rule::set_if_some;

use features_wallet_entities::wallet::{ActiveModel, ModelOptionDto};

pub fn assign(mut active_model: ActiveModel, model_option: ModelOptionDto) -> ActiveModel {
    set_if_some!(active_model.id, model_option.id);
    set_if_some!(active_model.user_id, model_option.user_id);
    set_if_some!(active_model.currency, model_option.currency);
    set_if_some!(active_model.balance, model_option.balance);
    set_if_some!(active_model.is_active, model_option.is_active);
    set_if_some!(active_model.created_at, model_option.created_at);
    set_if_some!(active_model.updated_at, model_option.updated_at);

    active_model
}
