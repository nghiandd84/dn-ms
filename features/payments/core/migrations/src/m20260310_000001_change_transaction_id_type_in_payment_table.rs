use sea_orm_migration::prelude::*;

use features_payments_core_entities::payment;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260310_000001_change_transaction_id_type_in_payment_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let _ = manager
            .alter_table(
                Table::alter()
                    .table(payment::Entity)
                    .modify_column(
                        ColumnDef::new(payment::Column::TransactionId)
                            .string()
                            .string_len(255)
                            .unique_key(),
                    )
                    .to_owned(),
            )
            .await;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let _ = manager
            .alter_table(
                Table::alter()
                    .table(payment::Entity)
                    .modify_column(ColumnDef::new(payment::Column::TransactionId).string())
                    .to_owned(),
            )
            .await;

        Ok(())
    }
}
