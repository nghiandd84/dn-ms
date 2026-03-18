use sea_orm_migration::prelude::*;

use features_merchant_entities::merchant;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260318_000001_create_merchant_tables"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let _create_merchant_table = manager
            .create_table(
                Table::create()
                    .table(merchant::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(merchant::Column::Id)
                            .string()
                            .string_len(50)
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(merchant::Column::BusinessName)
                            .string()
                            .string_len(255)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(merchant::Column::Email)
                            .string()
                            .string_len(255)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(merchant::Column::Phone)
                            .string()
                            .string_len(20),
                    )
                    .col(
                        ColumnDef::new(merchant::Column::BusinessType)
                            .string()
                            .string_len(50),
                    )
                    .col(
                        ColumnDef::new(merchant::Column::KycStatus)
                            .string()
                            .string_len(20)
                            .not_null(),
                    )
                    .col(ColumnDef::new(merchant::Column::KycVerifiedAt).date_time())
                    .col(
                        ColumnDef::new(merchant::Column::Status)
                            .string()
                            .string_len(20)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(merchant::Column::CreatedAt)
                            .date_time()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP"),
                    )
                    .col(
                        ColumnDef::new(merchant::Column::UpdatedAt)
                            .date_time()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP"),
                    )
                    .to_owned(),
            )
            .await;

        let _idx_email = manager
            .create_index(
                Index::create()
                    .name("idx_email")
                    .table(merchant::Entity)
                    .col(merchant::Column::Email)
                    .to_owned(),
            )
            .await;
        let _idx_kyc_status = manager
            .create_index(
                Index::create()
                    .name("idx_kyc_status")
                    .table(merchant::Entity)
                    .col(merchant::Column::KycStatus)
                    .to_owned(),
            )
            .await;
        let _idx_status = manager
            .create_index(
                Index::create()
                    .name("idx_status")
                    .table(merchant::Entity)
                    .col(merchant::Column::Status)
                    .to_owned(),
            )
            .await;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let _drop_idx_email = manager
            .drop_index(
                Index::drop()
                    .name("idx_email")
                    .table(merchant::Entity)
                    .to_owned(),
            )
            .await;
        let _drop_idx_kyc_status = manager
            .drop_index(
                Index::drop()
                    .name("idx_kyc_status")
                    .table(merchant::Entity)
                    .to_owned(),
            )
            .await;
        let _drop_idx_status = manager
            .drop_index(
                Index::drop()
                    .name("idx_status")
                    .table(merchant::Entity)
                    .to_owned(),
            )
            .await;

        let _drop_merchants_table = manager
            .drop_table(Table::drop().table(merchant::Entity).to_owned())
            .await;
        Ok(())
    }
}