use sea_orm_migration::{prelude::*, sea_orm::ColumnTrait};

use features_payments_stripe_entities::{
    stripe_api_log, stripe_payment_intent, stripe_refund, stripe_webhook_event,
};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260212_000001_create_stripe_tables"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create stripe_payment_intents table
        let _create_stripe_payment_intents_table = manager
            .create_table(
                Table::create()
                    .table(stripe_payment_intent::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(stripe_payment_intent::Column::Id)
                            .uuid()
                            .extra("DEFAULT public.uuid_generate_v4()")
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(stripe_payment_intent::Column::PaymentId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(stripe_payment_intent::Column::StripePaymentIntentId)
                            .string()
                            .string_len(255)
                            .not_null(),
                        // .unique_key(), // Can't be unique because we create the record before we have the Stripe ID, then update it later
                        // .unique_key(),
                    )
                    .col(
                        ColumnDef::new(stripe_payment_intent::Column::Amount)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(stripe_payment_intent::Column::Currency)
                            .string()
                            .string_len(3)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(stripe_payment_intent::Column::Status)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(stripe_payment_intent::Column::ClientSecret)
                            .string()
                            .string_len(255),
                    )
                    .col(ColumnDef::new(stripe_payment_intent::Column::Metadata).json())
                    .col(
                        ColumnDef::new(stripe_payment_intent::Column::CreatedAt)
                            .date_time()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP"),
                    )
                    .col(
                        ColumnDef::new(stripe_payment_intent::Column::UpdatedAt)
                            .date_time()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP"),
                    )
                    .to_owned(),
            )
            .await;

        // Create stripe_refunds table
        let _create_stripe_refunds_table = manager
            .create_table(
                Table::create()
                    .table(stripe_refund::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(stripe_refund::Column::Id)
                            .uuid()
                            .extra("DEFAULT public.uuid_generate_v4()")
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(stripe_refund::Column::PaymentId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(stripe_refund::Column::StripeRefundId)
                            .string()
                            .string_len(255)
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(stripe_refund::Column::StripePaymentIntentId)
                            .string()
                            .string_len(255)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(stripe_refund::Column::Amount)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(stripe_refund::Column::Currency)
                            .string()
                            .string_len(3)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(stripe_refund::Column::Status)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(stripe_refund::Column::Reason)
                            .string()
                            .string_len(50),
                    )
                    .col(ColumnDef::new(stripe_refund::Column::Metadata).json())
                    .col(
                        ColumnDef::new(stripe_refund::Column::CreatedAt)
                            .date_time()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP"),
                    )
                    .col(
                        ColumnDef::new(stripe_refund::Column::UpdatedAt)
                            .date_time()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP"),
                    )
                    .to_owned(),
            )
            .await;

        // Create stripe_webhook_events table
        let _create_stripe_webhook_events_table = manager
            .create_table(
                Table::create()
                    .table(stripe_webhook_event::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(stripe_webhook_event::Column::Id)
                            .uuid()
                            .extra("DEFAULT public.uuid_generate_v4()")
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(stripe_webhook_event::Column::StripeEventId)
                            .string()
                            .string_len(255)
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(stripe_webhook_event::Column::EventType)
                            .string()
                            .string_len(100)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(stripe_webhook_event::Column::EventData)
                            .json()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(stripe_webhook_event::Column::Processed)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(ColumnDef::new(stripe_webhook_event::Column::ProcessingError).text())
                    .col(
                        ColumnDef::new(stripe_webhook_event::Column::CreatedAt)
                            .date_time()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP"),
                    )
                    .col(
                        ColumnDef::new(stripe_webhook_event::Column::UpdatedAt)
                            .date_time()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP"),
                    )
                    .to_owned(),
            )
            .await;

        // Create stripe_api_logs table
        let _create_stripe_api_logs_table = manager
            .create_table(
                Table::create()
                    .table(stripe_api_log::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(stripe_api_log::Column::Id)
                            .uuid()
                            .extra("DEFAULT public.uuid_generate_v4()")
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(stripe_api_log::Column::Endpoint)
                            .string()
                            .string_len(500)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(stripe_api_log::Column::Method)
                            .string()
                            .string_len(10)
                            .not_null(),
                    )
                    .col(ColumnDef::new(stripe_api_log::Column::RequestBody).text())
                    .col(ColumnDef::new(stripe_api_log::Column::ResponseBody).text())
                    .col(ColumnDef::new(stripe_api_log::Column::StatusCode).integer())
                    .col(ColumnDef::new(stripe_api_log::Column::ErrorMessage).text())
                    .col(
                        ColumnDef::new(stripe_api_log::Column::StripeRequestId)
                            .string()
                            .string_len(255),
                    )
                    .col(
                        ColumnDef::new(stripe_api_log::Column::CreatedAt)
                            .date_time()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP"),
                    )
                    .col(
                        ColumnDef::new(stripe_api_log::Column::UpdatedAt)
                            .date_time()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP"),
                    )
                    .to_owned(),
            )
            .await;

        // Create indexes
        let _idx_stripe_payment_intent_payment_id = manager
            .create_index(
                Index::create()
                    .name("idx_stripe_payment_intent_payment_id")
                    .table(stripe_payment_intent::Entity)
                    .col(stripe_payment_intent::Column::PaymentId)
                    .to_owned(),
            )
            .await;

        let _idx_stripe_payment_intent_id =
            manager
                .create_index(
                    Index::create()
                        .name("idx_stripe_payment_intent_id")
                        .table(stripe_payment_intent::Entity)
                        .col(stripe_payment_intent::Column::StripePaymentIntentId)
                        .cond_where(Condition::any().add(
                            stripe_payment_intent::Column::StripePaymentIntentId.is_not_null(),
                        ))
                        .to_owned(),
                )
                .await;

        let _idx_stripe_refund_payment_id = manager
            .create_index(
                Index::create()
                    .name("idx_stripe_refund_payment_id")
                    .table(stripe_refund::Entity)
                    .col(stripe_refund::Column::PaymentId)
                    .to_owned(),
            )
            .await;

        let _idx_stripe_refund_payment_intent_id = manager
            .create_index(
                Index::create()
                    .name("idx_stripe_refund_payment_intent_id")
                    .table(stripe_refund::Entity)
                    .col(stripe_refund::Column::StripePaymentIntentId)
                    .to_owned(),
            )
            .await;

        let _idx_stripe_webhook_event_type = manager
            .create_index(
                Index::create()
                    .name("idx_stripe_webhook_event_type")
                    .table(stripe_webhook_event::Entity)
                    .col(stripe_webhook_event::Column::EventType)
                    .to_owned(),
            )
            .await;

        let _idx_stripe_webhook_processed = manager
            .create_index(
                Index::create()
                    .name("idx_stripe_webhook_processed")
                    .table(stripe_webhook_event::Entity)
                    .col(stripe_webhook_event::Column::Processed)
                    .to_owned(),
            )
            .await;

        let _idx_stripe_api_log_endpoint = manager
            .create_index(
                Index::create()
                    .name("idx_stripe_api_log_endpoint")
                    .table(stripe_api_log::Entity)
                    .col(stripe_api_log::Column::Endpoint)
                    .to_owned(),
            )
            .await;

        let _idx_stripe_api_log_created_at = manager
            .create_index(
                Index::create()
                    .name("idx_stripe_api_log_created_at")
                    .table(stripe_api_log::Entity)
                    .col(stripe_api_log::Column::CreatedAt)
                    .to_owned(),
            )
            .await;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop indexes
        let _drop_idx_stripe_payment_intent_payment_id = manager
            .drop_index(
                Index::drop()
                    .name("idx_stripe_payment_intent_payment_id")
                    .table(stripe_payment_intent::Entity)
                    .to_owned(),
            )
            .await;

        let _drop_idx_stripe_payment_intent_id = manager
            .drop_index(
                Index::drop()
                    .name("idx_stripe_payment_intent_id")
                    .table(stripe_payment_intent::Entity)
                    .to_owned(),
            )
            .await;

        let _drop_idx_stripe_refund_payment_id = manager
            .drop_index(
                Index::drop()
                    .name("idx_stripe_refund_payment_id")
                    .table(stripe_refund::Entity)
                    .to_owned(),
            )
            .await;

        let _drop_idx_stripe_refund_payment_intent_id = manager
            .drop_index(
                Index::drop()
                    .name("idx_stripe_refund_payment_intent_id")
                    .table(stripe_refund::Entity)
                    .to_owned(),
            )
            .await;

        let _drop_idx_stripe_webhook_event_type = manager
            .drop_index(
                Index::drop()
                    .name("idx_stripe_webhook_event_type")
                    .table(stripe_webhook_event::Entity)
                    .to_owned(),
            )
            .await;

        let _drop_idx_stripe_webhook_processed = manager
            .drop_index(
                Index::drop()
                    .name("idx_stripe_webhook_processed")
                    .table(stripe_webhook_event::Entity)
                    .to_owned(),
            )
            .await;

        let _drop_idx_stripe_api_log_endpoint = manager
            .drop_index(
                Index::drop()
                    .name("idx_stripe_api_log_endpoint")
                    .table(stripe_api_log::Entity)
                    .to_owned(),
            )
            .await;

        let _drop_idx_stripe_api_log_created_at = manager
            .drop_index(
                Index::drop()
                    .name("idx_stripe_api_log_created_at")
                    .table(stripe_api_log::Entity)
                    .to_owned(),
            )
            .await;

        // Drop tables
        let _drop_stripe_payment_intents_table = manager
            .drop_table(
                Table::drop()
                    .table(stripe_payment_intent::Entity)
                    .to_owned(),
            )
            .await;

        let _drop_stripe_refunds_table = manager
            .drop_table(Table::drop().table(stripe_refund::Entity).to_owned())
            .await;

        let _drop_stripe_webhook_events_table = manager
            .drop_table(Table::drop().table(stripe_webhook_event::Entity).to_owned())
            .await;

        let _drop_stripe_api_logs_table = manager
            .drop_table(Table::drop().table(stripe_api_log::Entity).to_owned())
            .await;

        Ok(())
    }
}
