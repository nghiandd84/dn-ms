use sea_orm_migration::prelude::*;

use features_fee_entities::fee_configuration;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
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
                    .col(
                        ColumnDef::new(fee_configuration::Column::PercentageRate)
                            .double()
                            .decimal_len(5, 2),
                    )
                    .col(
                        ColumnDef::new(fee_configuration::Column::FixedAmount)
                            .double()
                            .decimal_len(10, 2),
                    )
                    .col(
                        ColumnDef::new(fee_configuration::Column::MinFee)
                            .double()
                            .decimal_len(10, 2),
                    )
                    .col(
                        ColumnDef::new(fee_configuration::Column::MaxFee)
                            .double()
                            .decimal_len(10, 2),
                    )
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
                    .index(
                        Index::create()
                            .name("idx_merchant_id")
                            .table(fee_configuration::Entity)
                            .col(fee_configuration::Column::MerchantId),
                    )
                    .index(
                        Index::create()
                            .name("idx_effective_from")
                            .table(fee_configuration::Entity)
                            .col(fee_configuration::Column::EffectiveFrom),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(fee_configuration::Entity).to_owned())
            .await
    }
}
