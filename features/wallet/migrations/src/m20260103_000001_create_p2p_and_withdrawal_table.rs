use sea_orm_migration::prelude::*;

use features_wallet_entities::{p2p_transfer, withdrawal};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260103_000001_create_p2p_and_withdrawal_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let _ = manager
            .create_table(
                Table::create()
                    .table(p2p_transfer::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(p2p_transfer::Column::Id)
                            .uuid()
                            .extra("DEFAULT gen_random_uuid()")
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(p2p_transfer::Column::FromWalletId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(p2p_transfer::Column::ToWalletId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(p2p_transfer::Column::Amount)
                            .float()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(p2p_transfer::Column::Status)
                            .string()
                            .not_null()
                            .default("PENDING"),
                    )
                    .col(
                        ColumnDef::new(p2p_transfer::Column::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(p2p_transfer::Column::UpdatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(p2p_transfer::Column::CompletedAt)
                            .date_time()
                            .null(),
                    )
                    .to_owned(),
            )
            .await;

        let _ = manager
            .create_index(
                Index::create()
                    .table(p2p_transfer::Entity)
                    .name("idx_p2p_transfer_from_wallet")
                    .col(p2p_transfer::Column::FromWalletId)
                    .to_owned(),
            )
            .await;

        let _ = manager
            .create_index(
                Index::create()
                    .table(p2p_transfer::Entity)
                    .name("idx_p2p_transfer_to_wallet")
                    .col(p2p_transfer::Column::ToWalletId)
                    .to_owned(),
            )
            .await;

        let _ = manager
            .create_index(
                Index::create()
                    .table(p2p_transfer::Entity)
                    .name("idx_p2p_transfer_status")
                    .col(p2p_transfer::Column::Status)
                    .to_owned(),
            )
            .await;

        let _ = manager
            .create_table(
                Table::create()
                    .table(withdrawal::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(withdrawal::Column::Id)
                            .uuid()
                            .extra("DEFAULT gen_random_uuid()")
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(withdrawal::Column::WalletId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(withdrawal::Column::Amount)
                            .float()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(withdrawal::Column::PaymentDeviceId)
                            .uuid()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(withdrawal::Column::Status)
                            .string()
                            .not_null()
                            .default("PENDING"),
                    )
                    .col(
                        ColumnDef::new(withdrawal::Column::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(withdrawal::Column::UpdatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(withdrawal::Column::CompletedAt)
                            .date_time()
                            .null(),
                    )
                    .to_owned(),
            )
            .await;

        let _ = manager
            .create_index(
                Index::create()
                    .table(withdrawal::Entity)
                    .name("idx_withdrawal_wallet_id")
                    .col(withdrawal::Column::WalletId)
                    .to_owned(),
            )
            .await;

        let _ = manager
            .create_index(
                Index::create()
                    .table(withdrawal::Entity)
                    .name("idx_withdrawal_status")
                    .col(withdrawal::Column::Status)
                    .to_owned(),
            )
            .await;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(withdrawal::Entity).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(p2p_transfer::Entity).to_owned())
            .await?;
        Ok(())
    }
}
