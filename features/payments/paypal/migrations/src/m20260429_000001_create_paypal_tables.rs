use sea_orm_migration::{prelude::*, sea_orm::ColumnTrait};

use features_payments_paypal_entities::{
    paypal_api_log, paypal_order, paypal_refund, paypal_webhook_event,
};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260429_000001_create_paypal_tables"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create paypal_orders table
        let _create_paypal_orders_table = manager
            .create_table(
                Table::create()
                    .table(paypal_order::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(paypal_order::Column::Id)
                            .uuid()
                            .extra("DEFAULT gen_random_uuid()")
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(paypal_order::Column::PaymentId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(paypal_order::Column::PaypalOrderId)
                            .string()
                            .string_len(255)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(paypal_order::Column::Amount)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(paypal_order::Column::Currency)
                            .string()
                            .string_len(3)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(paypal_order::Column::Status)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(paypal_order::Column::ApprovalUrl)
                            .string()
                            .string_len(2000),
                    )
                    .col(
                        ColumnDef::new(paypal_order::Column::CaptureId)
                            .string()
                            .string_len(255),
                    )
                    .col(ColumnDef::new(paypal_order::Column::Metadata).json())
                    .col(
                        ColumnDef::new(paypal_order::Column::CreatedAt)
                            .date_time()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP"),
                    )
                    .col(
                        ColumnDef::new(paypal_order::Column::UpdatedAt)
                            .date_time()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP"),
                    )
                    .to_owned(),
            )
            .await;

        // Create paypal_refunds table
        let _create_paypal_refunds_table = manager
            .create_table(
                Table::create()
                    .table(paypal_refund::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(paypal_refund::Column::Id)
                            .uuid()
                            .extra("DEFAULT gen_random_uuid()")
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(paypal_refund::Column::PaymentId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(paypal_refund::Column::PaypalRefundId)
                            .string()
                            .string_len(255)
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(paypal_refund::Column::PaypalCaptureId)
                            .string()
                            .string_len(255)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(paypal_refund::Column::Amount)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(paypal_refund::Column::Currency)
                            .string()
                            .string_len(3)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(paypal_refund::Column::Status)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(paypal_refund::Column::Reason)
                            .string()
                            .string_len(500),
                    )
                    .col(ColumnDef::new(paypal_refund::Column::Metadata).json())
                    .col(
                        ColumnDef::new(paypal_refund::Column::CreatedAt)
                            .date_time()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP"),
                    )
                    .col(
                        ColumnDef::new(paypal_refund::Column::UpdatedAt)
                            .date_time()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP"),
                    )
                    .to_owned(),
            )
            .await;

        // Create paypal_webhook_events table
        let _create_paypal_webhook_events_table = manager
            .create_table(
                Table::create()
                    .table(paypal_webhook_event::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(paypal_webhook_event::Column::Id)
                            .uuid()
                            .extra("DEFAULT gen_random_uuid()")
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(paypal_webhook_event::Column::PaypalEventId)
                            .string()
                            .string_len(255)
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(paypal_webhook_event::Column::EventType)
                            .string()
                            .string_len(100)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(paypal_webhook_event::Column::EventData)
                            .json()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(paypal_webhook_event::Column::Processed)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(ColumnDef::new(paypal_webhook_event::Column::ProcessingError).text())
                    .col(
                        ColumnDef::new(paypal_webhook_event::Column::CreatedAt)
                            .date_time()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP"),
                    )
                    .col(
                        ColumnDef::new(paypal_webhook_event::Column::UpdatedAt)
                            .date_time()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP"),
                    )
                    .to_owned(),
            )
            .await;

        // Create paypal_api_logs table
        let _create_paypal_api_logs_table = manager
            .create_table(
                Table::create()
                    .table(paypal_api_log::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(paypal_api_log::Column::Id)
                            .uuid()
                            .extra("DEFAULT gen_random_uuid()")
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(paypal_api_log::Column::Endpoint)
                            .string()
                            .string_len(500)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(paypal_api_log::Column::Method)
                            .string()
                            .string_len(10)
                            .not_null(),
                    )
                    .col(ColumnDef::new(paypal_api_log::Column::RequestBody).text())
                    .col(ColumnDef::new(paypal_api_log::Column::ResponseBody).text())
                    .col(ColumnDef::new(paypal_api_log::Column::StatusCode).integer())
                    .col(ColumnDef::new(paypal_api_log::Column::ErrorMessage).text())
                    .col(
                        ColumnDef::new(paypal_api_log::Column::PaypalRequestId)
                            .string()
                            .string_len(255),
                    )
                    .col(
                        ColumnDef::new(paypal_api_log::Column::CreatedAt)
                            .date_time()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP"),
                    )
                    .col(
                        ColumnDef::new(paypal_api_log::Column::UpdatedAt)
                            .date_time()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP"),
                    )
                    .to_owned(),
            )
            .await;

        // Create indexes
        let _idx_paypal_order_payment_id = manager
            .create_index(
                Index::create()
                    .name("idx_paypal_order_payment_id")
                    .table(paypal_order::Entity)
                    .col(paypal_order::Column::PaymentId)
                    .to_owned(),
            )
            .await;

        let _idx_paypal_order_id = manager
            .create_index(
                Index::create()
                    .name("idx_paypal_order_paypal_order_id")
                    .table(paypal_order::Entity)
                    .col(paypal_order::Column::PaypalOrderId)
                    .cond_where(
                        Condition::any()
                            .add(paypal_order::Column::PaypalOrderId.is_not_null()),
                    )
                    .to_owned(),
            )
            .await;

        let _idx_paypal_refund_payment_id = manager
            .create_index(
                Index::create()
                    .name("idx_paypal_refund_payment_id")
                    .table(paypal_refund::Entity)
                    .col(paypal_refund::Column::PaymentId)
                    .to_owned(),
            )
            .await;

        let _idx_paypal_refund_capture_id = manager
            .create_index(
                Index::create()
                    .name("idx_paypal_refund_capture_id")
                    .table(paypal_refund::Entity)
                    .col(paypal_refund::Column::PaypalCaptureId)
                    .to_owned(),
            )
            .await;

        let _idx_paypal_webhook_event_type = manager
            .create_index(
                Index::create()
                    .name("idx_paypal_webhook_event_type")
                    .table(paypal_webhook_event::Entity)
                    .col(paypal_webhook_event::Column::EventType)
                    .to_owned(),
            )
            .await;

        let _idx_paypal_webhook_processed = manager
            .create_index(
                Index::create()
                    .name("idx_paypal_webhook_processed")
                    .table(paypal_webhook_event::Entity)
                    .col(paypal_webhook_event::Column::Processed)
                    .to_owned(),
            )
            .await;

        let _idx_paypal_api_log_endpoint = manager
            .create_index(
                Index::create()
                    .name("idx_paypal_api_log_endpoint")
                    .table(paypal_api_log::Entity)
                    .col(paypal_api_log::Column::Endpoint)
                    .to_owned(),
            )
            .await;

        let _idx_paypal_api_log_created_at = manager
            .create_index(
                Index::create()
                    .name("idx_paypal_api_log_created_at")
                    .table(paypal_api_log::Entity)
                    .col(paypal_api_log::Column::CreatedAt)
                    .to_owned(),
            )
            .await;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop indexes
        let _ = manager.drop_index(Index::drop().name("idx_paypal_order_payment_id").table(paypal_order::Entity).to_owned()).await;
        let _ = manager.drop_index(Index::drop().name("idx_paypal_order_paypal_order_id").table(paypal_order::Entity).to_owned()).await;
        let _ = manager.drop_index(Index::drop().name("idx_paypal_refund_payment_id").table(paypal_refund::Entity).to_owned()).await;
        let _ = manager.drop_index(Index::drop().name("idx_paypal_refund_capture_id").table(paypal_refund::Entity).to_owned()).await;
        let _ = manager.drop_index(Index::drop().name("idx_paypal_webhook_event_type").table(paypal_webhook_event::Entity).to_owned()).await;
        let _ = manager.drop_index(Index::drop().name("idx_paypal_webhook_processed").table(paypal_webhook_event::Entity).to_owned()).await;
        let _ = manager.drop_index(Index::drop().name("idx_paypal_api_log_endpoint").table(paypal_api_log::Entity).to_owned()).await;
        let _ = manager.drop_index(Index::drop().name("idx_paypal_api_log_created_at").table(paypal_api_log::Entity).to_owned()).await;

        // Drop tables
        let _ = manager.drop_table(Table::drop().table(paypal_order::Entity).to_owned()).await;
        let _ = manager.drop_table(Table::drop().table(paypal_refund::Entity).to_owned()).await;
        let _ = manager.drop_table(Table::drop().table(paypal_webhook_event::Entity).to_owned()).await;
        let _ = manager.drop_table(Table::drop().table(paypal_api_log::Entity).to_owned()).await;

        Ok(())
    }
}
