use sea_orm_migration::prelude::*;

use features_wallet_entities::top_up_transaction;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260102_000001_create_top_up_transaction_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let _create_top_up_transaction_table = manager
            .create_table(
                Table::create()
                    .table(top_up_transaction::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(top_up_transaction::Column::Id)
                            .uuid()
                            .extra("DEFAULT public.uuid_generate_v4()")
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(top_up_transaction::Column::WalletId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(top_up_transaction::Column::Amount)
                            .float()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(top_up_transaction::Column::Method)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(top_up_transaction::Column::PaymentProviderId)
                            .string()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(top_up_transaction::Column::PaymentTransactionId)
                            .string()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(top_up_transaction::Column::Status)
                            .string()
                            .not_null()
                            .default("PENDING"),
                    )
                    .col(
                        ColumnDef::new(top_up_transaction::Column::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(top_up_transaction::Column::UpdatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(top_up_transaction::Column::CompletedAt)
                            .date_time()
                            .null(),
                    )
                    .to_owned(),
            )
            .await;

        let _create_idx_top_up_status_index = manager
            .create_index(
                Index::create()
                    .table(top_up_transaction::Entity)
                    .name("idx_top_up_status")
                    .col(top_up_transaction::Column::Status)
                    .to_owned(),
            )
            .await;

        let _create_idx_top_up_created_at_index = manager
            .create_index(
                Index::create()
                    .table(top_up_transaction::Entity)
                    .name("idx_top_up_created_at")
                    .col(top_up_transaction::Column::CreatedAt)
                    .to_owned(),
            )
            .await;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let _drop_top_up_transaction_table = manager
            .drop_table(Table::drop().table(top_up_transaction::Entity).to_owned())
            .await?;
        let _drop_idx_top_up_status_index = manager
            .drop_index(
                Index::drop()
                    .table(top_up_transaction::Entity)
                    .name("idx_top_up_status")
                    .to_owned(),
            )
            .await?;
        let _drop_idx_top_up_created_at_index = manager
            .drop_index(
                Index::drop()
                    .table(top_up_transaction::Entity)
                    .name("idx_top_up_created_at")
                    .to_owned(),
            )
            .await?;
        Ok(())
    }
}
