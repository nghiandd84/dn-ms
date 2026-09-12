use sea_orm_migration::prelude::*;

use features_booking_entities::{
    booking, booking_approval, booking_capacity, booking_dispatch, booking_item, booking_queue,
    booking_recurrence, booking_window,
};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260212_000001_create_booking_tables"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // ---- core: bookings ------------------------------------------------
        manager
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
                    .col(
                        ColumnDef::new(booking::Column::BookingType)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(booking::Column::BookingMode)
                            .string()
                            .not_null(),
                    )
                    .col(ColumnDef::new(booking::Column::ResourceType).string().null())
                    .col(ColumnDef::new(booking::Column::ResourceId).uuid().null())
                    .col(ColumnDef::new(booking::Column::UserId).uuid().not_null())
                    .col(
                        ColumnDef::new(booking::Column::TotalAmount)
                            .float()
                            .not_null()
                            .default(0.0),
                    )
                    .col(
                        ColumnDef::new(booking::Column::Currency)
                            .string()
                            .not_null()
                            .default("USD"),
                    )
                    .col(
                        ColumnDef::new(booking::Column::Status)
                            .string()
                            .not_null()
                            .default("PENDING"),
                    )
                    .col(ColumnDef::new(booking::Column::PaymentId).uuid().null())
                    .col(
                        ColumnDef::new(booking::Column::PaymentStatus)
                            .string()
                            .not_null()
                            .default("PENDING"),
                    )
                    .col(
                        ColumnDef::new(booking::Column::BookingReference)
                            .string_len(100)
                            .not_null(),
                    )
                    .col(ColumnDef::new(booking::Column::Metadata).json_binary().null())
                    .col(
                        ColumnDef::new(booking::Column::Version)
                            .integer()
                            .not_null()
                            .default(0),
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
                    .col(ColumnDef::new(booking::Column::ConfirmedAt).date_time().null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_bookings_user")
                    .table(booking::Entity)
                    .col(booking::Column::UserId)
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("idx_bookings_type_status")
                    .table(booking::Entity)
                    .col(booking::Column::BookingType)
                    .col(booking::Column::Status)
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("idx_bookings_resource")
                    .table(booking::Entity)
                    .col(booking::Column::ResourceType)
                    .col(booking::Column::ResourceId)
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("idx_bookings_reference")
                    .table(booking::Entity)
                    .col(booking::Column::BookingReference)
                    .to_owned(),
            )
            .await?;

        // ---- line items: booking_items ------------------------------------
        manager
            .create_table(
                Table::create()
                    .table(booking_item::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(booking_item::Column::Id)
                            .uuid()
                            .extra("DEFAULT gen_random_uuid()")
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(booking_item::Column::BookingId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(booking_item::Column::ItemType)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(booking_item::Column::ItemId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(booking_item::Column::Price)
                            .float()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(booking_item::Column::Metadata)
                            .json_binary()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(booking_item::Column::CreatedAt)
                            .date_time()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP"),
                    )
                    .col(
                        ColumnDef::new(booking_item::Column::UpdatedAt)
                            .date_time()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP"),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_booking_items_booking")
                            .from(booking_item::Entity, booking_item::Column::BookingId)
                            .to(booking::Entity, booking::Column::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_booking_items_booking")
                    .table(booking_item::Entity)
                    .col(booking_item::Column::BookingId)
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("idx_booking_items_unit")
                    .table(booking_item::Entity)
                    .col(booking_item::Column::ItemType)
                    .col(booking_item::Column::ItemId)
                    .to_owned(),
            )
            .await?;

        // ---- mode: booking_windows ----------------------------------------
        manager
            .create_table(
                Table::create()
                    .table(booking_window::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(booking_window::Column::BookingId)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(booking_window::Column::StartsAt)
                            .date_time()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(booking_window::Column::EndsAt)
                            .date_time()
                            .not_null(),
                    )
                    .col(ColumnDef::new(booking_window::Column::PartySize).integer().null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_booking_windows_booking")
                            .from(booking_window::Entity, booking_window::Column::BookingId)
                            .to(booking::Entity, booking::Column::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("idx_booking_windows_range")
                    .table(booking_window::Entity)
                    .col(booking_window::Column::StartsAt)
                    .col(booking_window::Column::EndsAt)
                    .to_owned(),
            )
            .await?;

        // ---- mode: booking_capacity ---------------------------------------
        manager
            .create_table(
                Table::create()
                    .table(booking_capacity::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(booking_capacity::Column::BookingId)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(booking_capacity::Column::ContainerId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(booking_capacity::Column::Quantity)
                            .integer()
                            .not_null()
                            .default(1),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_booking_capacity_booking")
                            .from(booking_capacity::Entity, booking_capacity::Column::BookingId)
                            .to(booking::Entity, booking::Column::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("idx_booking_capacity_container")
                    .table(booking_capacity::Entity)
                    .col(booking_capacity::Column::ContainerId)
                    .to_owned(),
            )
            .await?;

        // ---- mode: booking_recurrence -------------------------------------
        manager
            .create_table(
                Table::create()
                    .table(booking_recurrence::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(booking_recurrence::Column::BookingId)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(booking_recurrence::Column::Rrule)
                            .string_len(500)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(booking_recurrence::Column::ValidFrom)
                            .date_time()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(booking_recurrence::Column::ValidTo)
                            .date_time()
                            .null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_booking_recurrence_booking")
                            .from(
                                booking_recurrence::Entity,
                                booking_recurrence::Column::BookingId,
                            )
                            .to(booking::Entity, booking::Column::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // ---- mode: booking_approval ---------------------------------------
        manager
            .create_table(
                Table::create()
                    .table(booking_approval::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(booking_approval::Column::BookingId)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(booking_approval::Column::ApprovalStatus)
                            .string()
                            .not_null()
                            .default("PENDING"),
                    )
                    .col(
                        ColumnDef::new(booking_approval::Column::ApproverId)
                            .uuid()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(booking_approval::Column::RequestedAt)
                            .date_time()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP"),
                    )
                    .col(
                        ColumnDef::new(booking_approval::Column::DecidedAt)
                            .date_time()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(booking_approval::Column::HoldExpiresAt)
                            .date_time()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(booking_approval::Column::Reason)
                            .string_len(500)
                            .null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_booking_approval_booking")
                            .from(booking_approval::Entity, booking_approval::Column::BookingId)
                            .to(booking::Entity, booking::Column::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // ---- mode: booking_queue ------------------------------------------
        manager
            .create_table(
                Table::create()
                    .table(booking_queue::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(booking_queue::Column::BookingId)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(booking_queue::Column::QueueKey)
                            .string_len(100)
                            .not_null(),
                    )
                    .col(ColumnDef::new(booking_queue::Column::Position).integer().null())
                    .col(
                        ColumnDef::new(booking_queue::Column::EstimatedReady)
                            .date_time()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(booking_queue::Column::OfferedAt)
                            .date_time()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(booking_queue::Column::HoldExpiresAt)
                            .date_time()
                            .null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_booking_queue_booking")
                            .from(booking_queue::Entity, booking_queue::Column::BookingId)
                            .to(booking::Entity, booking::Column::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("idx_booking_queue_key_pos")
                    .table(booking_queue::Entity)
                    .col(booking_queue::Column::QueueKey)
                    .col(booking_queue::Column::Position)
                    .to_owned(),
            )
            .await?;

        // ---- mode: booking_dispatch ---------------------------------------
        manager
            .create_table(
                Table::create()
                    .table(booking_dispatch::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(booking_dispatch::Column::BookingId)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(booking_dispatch::Column::DispatchState)
                            .string()
                            .not_null()
                            .default("REQUESTED"),
                    )
                    .col(
                        ColumnDef::new(booking_dispatch::Column::ProviderId)
                            .uuid()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(booking_dispatch::Column::AssignedAt)
                            .date_time()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(booking_dispatch::Column::PickupLocation)
                            .json_binary()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(booking_dispatch::Column::DropoffLocation)
                            .json_binary()
                            .null(),
                    )
                    .col(ColumnDef::new(booking_dispatch::Column::Eta).date_time().null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_booking_dispatch_booking")
                            .from(booking_dispatch::Entity, booking_dispatch::Column::BookingId)
                            .to(booking::Entity, booking::Column::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("idx_booking_dispatch_provider")
                    .table(booking_dispatch::Entity)
                    .col(booking_dispatch::Column::ProviderId)
                    .col(booking_dispatch::Column::DispatchState)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop children first (FKs), then the core table.
        manager
            .drop_table(Table::drop().table(booking_dispatch::Entity).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(booking_queue::Entity).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(booking_approval::Entity).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(booking_recurrence::Entity).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(booking_capacity::Entity).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(booking_window::Entity).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(booking_item::Entity).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(booking::Entity).to_owned())
            .await?;
        Ok(())
    }
}
