use sea_orm::Set;

use shared_shared_macro_rule::set_if_some;

use features_wallet_entities::transaction::{ActiveModel, ModelOptionDto};

pub fn assign(mut active_model: ActiveModel, model_option: ModelOptionDto) -> ActiveModel {
    set_if_some!(active_model.id, model_option.id);
    set_if_some!(active_model.wallet_id, model_option.wallet_id);
    set_if_some!(active_model.transaction_type, model_option.transaction_type);
    set_if_some!(active_model.amount, model_option.amount);
    set_if_some!(active_model.currency, model_option.currency);
    set_if_some!(active_model.status, model_option.status);
    set_if_some!(active_model.reference_id, model_option.reference_id);
    set_if_some!(active_model.description, model_option.description);
    set_if_some!(active_model.created_at, model_option.created_at);
    set_if_some!(active_model.updated_at, model_option.updated_at);

    active_model
}
