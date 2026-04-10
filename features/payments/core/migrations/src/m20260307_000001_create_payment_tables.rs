use sea_orm_migration::prelude::*;

use features_payments_core_entities::{
    payment, payment_attempt, payment_method, payment_method_limit,
};

// Import Postgres-specific Type support for enums
// use sea_orm_migration::prelude::sea_query::extension::postgres::Type;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260307_000001_create_payment_tables"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // -- 1. Create enum type for payment_status (optional)
        /*
        let _payment_status_enum = manager
            .create_type(
                Type::create()
                    .as_enum(Alias::new("payment_status"))
                    .values([
                        Alias::new("created"),
                        Alias::new("processing"),
                        Alias::new("succeeded"),
                        Alias::new("failed"),
                        Alias::new("refunded"),
                    ])
                    .to_owned(),
            )
            .await;
        */

        // -- 2. payment_methods table
        let _ = manager
            .create_table(
                Table::create()
                    .table(payment_method::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(payment_method::Column::Id)
                            .uuid()
                            .extra("DEFAULT gen_random_uuid()")
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(payment_method::Column::DisplayName)
                            .string()
                            .string_len(100)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(payment_method::Column::ProviderName)
                            .string()
                            .string_len(50)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(payment_method::Column::ProviderConfig)
                            .json()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(payment_method::Column::SupportedCountries)
                            .array(ColumnType::String(StringLen::N(2)))
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(payment_method::Column::SupportedCurrencies)
                            .array(ColumnType::String(StringLen::N(3)))
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(payment_method::Column::Priority)
                            .integer()
                            .not_null()
                            .default(1),
                    )
                    .col(
                        ColumnDef::new(payment_method::Column::IsActive)
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .col(ColumnDef::new(payment_method::Column::FeePercentage).float())
                    .col(
                        ColumnDef::new(payment_method::Column::IconUrl)
                            .text()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(payment_method::Column::CreatedAt)
                            .date_time()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP"),
                    )
                    .col(
                        ColumnDef::new(payment_method::Column::UpdatedAt)
                            .date_time()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP"),
                    )
                    .to_owned(),
            )
            .await;

        // -- 2.5. payment_method_limits table
        let _ = manager
            .create_table(
                Table::create()
                    .table(payment_method_limit::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(payment_method_limit::Column::Id)
                            .uuid()
                            .extra("DEFAULT gen_random_uuid()")
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(payment_method_limit::Column::PaymentMethodId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(payment_method_limit::Column::Currency)
                            .string()
                            .string_len(3)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(payment_method_limit::Column::MinAmount)
                            .big_integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(payment_method_limit::Column::MaxAmount)
                            .big_integer()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(payment_method_limit::Column::CreatedAt)
                            .date_time()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP"),
                    )
                    .col(
                        ColumnDef::new(payment_method_limit::Column::UpdatedAt)
                            .date_time()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP"),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_payment_method_limits_payment_method_id")
                            .from(
                                payment_method_limit::Entity,
                                payment_method_limit::Column::PaymentMethodId,
                            )
                            .to(payment_method::Entity, payment_method::Column::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await;

        // -- 3. payments table
        let _ = manager
            .create_table(
                Table::create()
                    .table(payment::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(payment::Column::Id)
                            .uuid()
                            .extra("DEFAULT gen_random_uuid()")
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(payment::Column::TransactionId)
                            .uuid()
                            .not_null(),
                    )
                    .col(ColumnDef::new(payment::Column::UserId).uuid().not_null())
                    .col(
                        ColumnDef::new(payment::Column::Amount)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(payment::Column::Currency)
                            .string()
                            .string_len(3)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(payment::Column::Status)
                            .string()
                            .string_len(50)
                            .not_null()
                            .default("created"),
                    )
                    .col(
                        ColumnDef::new(payment::Column::ProviderName)
                            .string()
                            .string_len(50)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(payment::Column::GatewayTransactionId)
                            .string()
                            .string_len(255)
                            .unique_key()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(payment::Column::IdempotencyKey)
                            .string()
                            .string_len(255)
                            .unique_key()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(payment::Column::CreatedAt)
                            .date_time()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP"),
                    )
                    .col(
                        ColumnDef::new(payment::Column::UpdatedAt)
                            .date_time()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP"),
                    )
                    .to_owned(),
            )
            .await;

        // -- 4. payment_attempts table
        let _ = manager
            .create_table(
                Table::create()
                    .table(payment_attempt::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(payment_attempt::Column::Id)
                            .uuid()
                            .extra("DEFAULT gen_random_uuid()")
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(payment_attempt::Column::PaymentId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(payment_attempt::Column::Provider)
                            .string()
                            .string_len(50)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(payment_attempt::Column::RawResponse)
                            .json()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(payment_attempt::Column::Success)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(payment_attempt::Column::ErrorMessage)
                            .text()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(payment_attempt::Column::CreatedAt)
                            .date_time()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP"),
                    )
                    .to_owned(),
            )
            .await;

        // -- 5. indexes
        let _ = manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_pm_country")
                    .table(payment_method::Entity)
                    .col(payment_method::Column::SupportedCountries)
                    .to_owned(),
            )
            .await;

        let _ = manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_pm_active")
                    .table(payment_method::Entity)
                    .col(payment_method::Column::IsActive)
                    .to_owned(),
            )
            .await;

        let _ = manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_payments_transaction_id")
                    .table(payment::Entity)
                    .col(payment::Column::TransactionId)
                    .to_owned(),
            )
            .await;

        let _ = manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_payments_status")
                    .table(payment::Entity)
                    .col(payment::Column::Status)
                    .to_owned(),
            )
            .await;

        let _ = manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("unique_pm_limit")
                    .table(payment_method_limit::Entity)
                    .col(payment_method_limit::Column::PaymentMethodId)
                    .col(payment_method_limit::Column::Currency)
                    .unique()
                    .to_owned(),
            )
            .await;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let _ = manager
            .drop_index(
                Index::drop()
                    .if_exists()
                    .name("idx_pm_country")
                    .table(payment_method::Entity)
                    .to_owned(),
            )
            .await;
        let _ = manager
            .drop_index(
                Index::drop()
                    .if_exists()
                    .name("idx_pm_active")
                    .table(payment_method::Entity)
                    .to_owned(),
            )
            .await;
        let _ = manager
            .drop_index(
                Index::drop()
                    .if_exists()
                    .name("idx_payments_transaction_id")
                    .table(payment::Entity)
                    .to_owned(),
            )
            .await;
        let _ = manager
            .drop_index(
                Index::drop()
                    .if_exists()
                    .name("idx_payments_status")
                    .table(payment::Entity)
                    .to_owned(),
            )
            .await;

        let _ = manager
            .drop_index(
                Index::drop()
                    .if_exists()
                    .name("unique_pm_limit")
                    .table(payment_method_limit::Entity)
                    .to_owned(),
            )
            .await;

        let _ = manager
            .drop_table(
                Table::drop()
                    .if_exists()
                    .table(payment_attempt::Entity)
                    .to_owned(),
            )
            .await;
        let _ = manager
            .drop_table(Table::drop().if_exists().table(payment::Entity).to_owned())
            .await;
        let _ = manager
            .drop_table(
                Table::drop()
                    .if_exists()
                    .table(payment_method_limit::Entity)
                    .to_owned(),
            )
            .await;
        let _ = manager
            .drop_table(
                Table::drop()
                    .if_exists()
                    .table(payment_method::Entity)
                    .to_owned(),
            )
            .await;

        /*
        let _ = manager
            .drop_type(
                Type::drop()
                    .name(Alias::new("payment_status"))
                    .if_exists()
                    .to_owned(),
            )
            .await;
        */

        Ok(())
    }
}
