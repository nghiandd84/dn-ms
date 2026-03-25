use sea_orm_migration::prelude::*;

use features_wallet_entities::{wallet, transaction};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260101_000001_create_wallet_tables"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let _create_wallet_table = manager
            .create_table(
                Table::create()
                    .table(wallet::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(wallet::Column::Id)
                            .uuid()
                            .extra("DEFAULT public.uuid_generate_v4()")
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(wallet::Column::UserId).uuid().not_null())
                    .col(
                        ColumnDef::new(wallet::Column::Currency)
                            .string()
                            .not_null()
                            .default("USD"),
                    )
                    .col(
                        ColumnDef::new(wallet::Column::Balance)
                            .string()
                            .not_null()
                            .default("0"),
                    )
                    .col(
                        ColumnDef::new(wallet::Column::IsActive)
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .col(
                        ColumnDef::new(wallet::Column::CreatedAt)
                            .date_time()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP"),
                    )
                    .col(
                        ColumnDef::new(wallet::Column::UpdatedAt)
                            .date_time()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP"),
                    )
                    .to_owned(),
            )
            .await;

        let _create_transaction_table = manager
            .create_table(
                Table::create()
                    .table(transaction::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(transaction::Column::Id)
                            .uuid()
                            .extra("DEFAULT public.uuid_generate_v4()")
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(transaction::Column::WalletId).uuid().not_null())
                    .col(
                        ColumnDef::new(transaction::Column::TransactionType)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(transaction::Column::Amount)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(transaction::Column::Currency)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(transaction::Column::Status)
                            .string()
                            .not_null()
                            .default("PENDING"),
                    )
                    .col(
                        ColumnDef::new(transaction::Column::ReferenceId)
                            .string()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(transaction::Column::Description)
                            .string()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(transaction::Column::CreatedAt)
                            .date_time()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP"),
                    )
                    .col(
                        ColumnDef::new(transaction::Column::UpdatedAt)
                            .date_time()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP"),
                    )
                    .to_owned(),
            )
            .await;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(transaction::Entity).to_owned())
            .await?;
        
        manager
            .drop_table(Table::drop().table(wallet::Entity).to_owned())
            .await?;

        Ok(())
    }
}
