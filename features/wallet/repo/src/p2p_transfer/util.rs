use sea_orm::Set;

use shared_shared_macro_rule::set_if_some;

use features_wallet_entities::p2p_transfer::{ActiveModel, ModelOptionDto};

pub fn assign(mut active_model: ActiveModel, model_option: ModelOptionDto) -> ActiveModel {
    set_if_some!(active_model.id, model_option.id);
    set_if_some!(active_model.from_wallet_id, model_option.from_wallet_id);
    set_if_some!(active_model.to_wallet_id, model_option.to_wallet_id);
    set_if_some!(active_model.amount, model_option.amount);
    set_if_some!(active_model.status, model_option.status);
    set_if_some!(active_model.created_at, model_option.created_at);
    set_if_some!(active_model.updated_at, model_option.updated_at);
    set_if_some!(active_model.completed_at, model_option.completed_at);

    active_model
}
