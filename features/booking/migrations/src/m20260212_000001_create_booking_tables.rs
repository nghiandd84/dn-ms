use sea_orm_migration::prelude::*;

use features_booking_entities::{booking, booking_seat};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260212_000001_create_booking_tables"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let _create_booking_table = manager
            .create_table(
                Table::create()
                    .table(booking::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(booking::Column::Id)
                            .uuid()
                            .extra("DEFAULT gen_random_uuid()")
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(booking::Column::EventId).uuid().not_null())
                    .col(ColumnDef::new(booking::Column::UserId).uuid().not_null())
                    .col(
                        ColumnDef::new(booking::Column::TotalAmount)
                            .float()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(booking::Column::Status)
                            .string()
                            .not_null()
                            .default("PENDING"),
                    )
                    .col(ColumnDef::new(booking::Column::PaymentId).null().uuid())
                    .col(
                        ColumnDef::new(booking::Column::PaymentStatus)
                            .string()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(booking::Column::BookingReference)
                            .string()
                            .string_len(50)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(booking::Column::CreatedAt)
                            .date_time()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP"),
                    )
                    .col(
                        ColumnDef::new(booking::Column::UpdatedAt)
                            .date_time()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP"),
                    )
                    .col(ColumnDef::new(booking::Column::ConfirmedAt).date_time())
                    .to_owned(),
            )
            .await;

        let _create_booking_seat_table = manager
            .create_table(
                Table::create()
                    .table(booking_seat::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(booking_seat::Column::Id)
                            .uuid()
                            .extra("DEFAULT gen_random_uuid()")
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(booking_seat::Column::BookingId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(booking_seat::Column::SeatId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(booking_seat::Column::Price)
                            .float()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(booking_seat::Column::CreatedAt)
                            .date_time()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP"),
                    )
                    .col(
                        ColumnDef::new(booking_seat::Column::UpdatedAt)
                            .date_time()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP"),
                    )
                    .to_owned(),
            )
            .await;

        // Create unique index for booking_reference
        let _idx_booking_reference = manager
            .create_index(
                Index::create()
                    .name("idx_booking_reference")
                    .table(booking::Entity)
                    .col(booking::Column::BookingReference)
                    .unique()
                    .to_owned(),
            )
            .await;

        // Create index for user_id
        let _idx_user_id = manager
            .create_index(
                Index::create()
                    .name("idx_user_id")
                    .table(booking::Entity)
                    .col(booking::Column::UserId)
                    .to_owned(),
            )
            .await;

        // Create index for booking status
        let _idx_booking_status = manager
            .create_index(
                Index::create()
                    .name("idx_booking_status")
                    .table(booking::Entity)
                    .col(booking::Column::Status)
                    .to_owned(),
            )
            .await;

        // Create unique index for booking_seat combination
        let _uk_booking_seat = manager
            .create_index(
                Index::create()
                    .name("uk_booking_seat")
                    .table(booking_seat::Entity)
                    .col(booking_seat::Column::BookingId)
                    .col(booking_seat::Column::SeatId)
                    .unique()
                    .to_owned(),
            )
            .await;

        // Create index for seat_id
        let _idx_seat_id = manager
            .create_index(
                Index::create()
                    .name("idx_seat_id")
                    .table(booking_seat::Entity)
                    .col(booking_seat::Column::SeatId)
                    .to_owned(),
            )
            .await;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let _drop_booking_seat_table = manager
            .drop_table(Table::drop().table(booking_seat::Entity).to_owned())
            .await;

        let _drop_booking_table = manager
            .drop_table(Table::drop().table(booking::Entity).to_owned())
            .await;

        Ok(())
    }
}
