use sea_orm_migration::prelude::*;

use features_fee_entities::fee_configuration;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let _creat_table = manager
            .create_table(
                Table::create()
                    .table(fee_configuration::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(fee_configuration::Column::Id)
                            .uuid()
                            .extra("DEFAULT public.uuid_generate_v4()")
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(fee_configuration::Column::MerchantId)
                            .string()
                            .string_len(50)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(fee_configuration::Column::PricingModel)
                            .string()
                            .string_len(20)
                            .not_null(),
                    )
                    .col(ColumnDef::new(fee_configuration::Column::PercentageRate).float())
                    .col(ColumnDef::new(fee_configuration::Column::FixedAmount).float())
                    .col(ColumnDef::new(fee_configuration::Column::MinFee).float())
                    .col(ColumnDef::new(fee_configuration::Column::MaxFee).float())
                    .col(ColumnDef::new(fee_configuration::Column::TierConfig).json())
                    .col(
                        ColumnDef::new(fee_configuration::Column::EffectiveFrom)
                            .timestamp()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(fee_configuration::Column::EffectiveTo)
                            .timestamp()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(fee_configuration::Column::CreatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(fee_configuration::Column::UpdatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await;
        let _create_idx_merchant_id = manager
            .create_index(
                Index::create()
                    .name("idx_merchant_id")
                    .table(fee_configuration::Entity)
                    .col(fee_configuration::Column::MerchantId)
                    .to_owned(),
            )
            .await;
        let _create_idx_effective_from = manager
            .create_index(
                Index::create()
                    .name("idx_effective_from")
                    .table(fee_configuration::Entity)
                    .col(fee_configuration::Column::EffectiveFrom)
                    .to_owned(),
            )
            .await;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let _drop_idx_merchant_id = manager
            .drop_index(
                Index::drop()
                    .name("idx_merchant_id")
                    .table(fee_configuration::Entity)
                    .to_owned(),
            )
            .await;
        let _drop_idx_effective_from = manager
            .drop_index(
                Index::drop()
                    .name("idx_effective_from")
                    .table(fee_configuration::Entity)
                    .to_owned(),
            )
            .await;
        let _drop_table = manager
            .drop_table(Table::drop().table(fee_configuration::Entity).to_owned())
            .await;
        Ok(())
    }
}
