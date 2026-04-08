use sea_orm::Set;

use shared_shared_macro_rule::set_if_some;

use features_wallet_entities::top_up_transaction::{ActiveModel, ModelOptionDto};

pub fn assign(mut active_model: ActiveModel, model_option: ModelOptionDto) -> ActiveModel {
    set_if_some!(active_model.id, model_option.id);
    set_if_some!(active_model.wallet_id, model_option.wallet_id);
    set_if_some!(active_model.amount, model_option.amount);
    set_if_some!(active_model.method, model_option.method);
    set_if_some!(
        active_model.payment_provider_id,
        model_option.payment_provider_id
    );
    set_if_some!(
        active_model.payment_transaction_id,
        model_option.payment_transaction_id
    );
    set_if_some!(active_model.status, model_option.status);
    set_if_some!(active_model.created_at, model_option.created_at);
    set_if_some!(active_model.updated_at, model_option.updated_at);
    set_if_some!(active_model.completed_at, model_option.completed_at);

    active_model
}
