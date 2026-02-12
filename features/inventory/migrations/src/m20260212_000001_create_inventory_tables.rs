use sea_orm_migration::prelude::*;

use features_inventory_entities::{reservation, seat};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260212_000001_create_inventory_tables"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let _create_seat_table = manager
            .create_table(
                Table::create()
                    .table(seat::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(seat::Column::Id)
                            .uuid()
                            .extra("DEFAULT public.uuid_generate_v4()")
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(seat::Column::EventId).uuid().not_null())
                    .col(
                        ColumnDef::new(seat::Column::SeatNumber)
                            .string()
                            .string_len(255)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(seat::Column::Section)
                            .string()
                            .string_len(255),
                    )
                    .col(
                        ColumnDef::new(seat::Column::RowNumber)
                            .string()
                            .string_len(255),
                    )
                    .col(
                        ColumnDef::new(seat::Column::SeatType)
                            .string()
                            .not_null()
                            .default("REGULAR"),
                    )
                    .col(ColumnDef::new(seat::Column::Price).decimal().not_null())
                    .col(
                        ColumnDef::new(seat::Column::Version)
                            .big_integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(seat::Column::Status)
                            .string()
                            .not_null()
                            .default("AVAILABLE"),
                    )
                    .col(ColumnDef::new(seat::Column::ReservedBy).string())
                    .col(ColumnDef::new(seat::Column::ReservedUntil).date_time())
                    .col(ColumnDef::new(seat::Column::BookingId).uuid())
                    .col(
                        ColumnDef::new(seat::Column::CreatedAt)
                            .date_time()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP"),
                    )
                    .to_owned(),
            )
            .await;
        let _create_reservation_table = manager
            .create_table(
                Table::create()
                    .table(reservation::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(reservation::Column::Id)
                            .uuid()
                            .extra("DEFAULT public.uuid_generate_v4()")
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(reservation::Column::SeatId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(reservation::Column::EventId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(reservation::Column::UserId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(reservation::Column::ExpiresAt)
                            .date_time()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(reservation::Column::Status)
                            .string()
                            .not_null()
                            .default("ACTIVE"),
                    )
                    .col(
                        ColumnDef::new(reservation::Column::CreatedAt)
                            .date_time()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP"),
                    )
                    .to_owned(),
            )
            .await;

        let _idx_event_status = manager
            .create_index(
                Index::create()
                    .name("idx_event_status")
                    .table(seat::Entity)
                    .col(seat::Column::EventId)
                    .col(seat::Column::Status)
                    .to_owned(),
            )
            .await;
        let _idx_reserved_until = manager
            .create_index(
                Index::create()
                    .name("idx_reserved_until")
                    .table(seat::Entity)
                    .col(seat::Column::ReservedUntil)
                    .to_owned(),
            )
            .await;
        let _idx_seat_id = manager
            .create_index(
                Index::create()
                    .name("idx_seat_id")
                    .table(reservation::Entity)
                    .col(reservation::Column::SeatId)
                    .to_owned(),
            )
            .await;
        let _idx_expires_at = manager
            .create_index(
                Index::create()
                    .name("idx_expires_at")
                    .table(reservation::Entity)
                    .col(reservation::Column::ExpiresAt)
                    .to_owned(),
            )
            .await;
        let _idx_user_id = manager
            .create_index(
                Index::create()
                    .name("idx_user_id")
                    .table(reservation::Entity)
                    .col(reservation::Column::UserId)
                    .to_owned(),
            )
            .await;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let _drop_idx_event_status = manager
            .drop_index(
                Index::drop()
                    .name("idx_event_status")
                    .table(seat::Entity)
                    .to_owned(),
            )
            .await;
        let _drop_idx_reserved_until = manager
            .drop_index(
                Index::drop()
                    .name("idx_reserved_until")
                    .table(seat::Entity)
                    .to_owned(),
            )
            .await;
        let _drop_idx_seat_id = manager
            .drop_index(
                Index::drop()
                    .name("idx_seat_id")
                    .table(reservation::Entity)
                    .to_owned(),
            )
            .await;
        let _drop_idx_expires_at = manager
            .drop_index(
                Index::drop()
                    .name("idx_expires_at")
                    .table(reservation::Entity)
                    .to_owned(),
            )
            .await;
        let _drop_idx_user_id = manager
            .drop_index(
                Index::drop()
                    .name("idx_user_id")
                    .table(reservation::Entity)
                    .to_owned(),
            )
            .await;

        let _drop_seats_table = manager
            .drop_table(Table::drop().table(seat::Entity).to_owned())
            .await;
        let _drop_reservations_table = manager
            .drop_table(Table::drop().table(reservation::Entity).to_owned())
            .await;
        Ok(())
    }
}
