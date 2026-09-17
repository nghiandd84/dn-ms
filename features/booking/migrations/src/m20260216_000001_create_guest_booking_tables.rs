use sea_orm_migration::prelude::*;

use features_booking_entities::{guest_booking, guest_booking_item};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260216_000001_create_guest_booking_tables"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // ---- guest_bookings ------------------------------------------------
        manager
            .create_table(
                Table::create()
                    .table(guest_booking::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(guest_booking::Column::Id)
                            .uuid()
                            .extra("DEFAULT gen_random_uuid()")
                            .not_null()
                            .primary_key(),
                    )
                    // classification
                    .col(
                        ColumnDef::new(guest_booking::Column::BookingType)
                            .string()
                            .not_null()
                            .default("EVENT"),
                    )
                    .col(
                        ColumnDef::new(guest_booking::Column::BookingMode)
                            .string()
                            .not_null()
                            .default("CAPACITY"),
                    )
                    // polymorphic target (owned by another service)
                    .col(
                        ColumnDef::new(guest_booking::Column::ResourceType)
                            .string()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(guest_booking::Column::ResourceId)
                            .uuid()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(guest_booking::Column::ExternalRef)
                            .string()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(guest_booking::Column::SiteOrigin)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(guest_booking::Column::ConfirmPath)
                            .string_len(255)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(guest_booking::Column::GuestEmail)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(guest_booking::Column::GuestName)
                            .string()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(guest_booking::Column::TotalAmount)
                            .float()
                            .not_null()
                            .default(0.0),
                    )
                    .col(
                        ColumnDef::new(guest_booking::Column::Currency)
                            .string()
                            .not_null()
                            .default("USD"),
                    )
                    .col(
                        ColumnDef::new(guest_booking::Column::Status)
                            .string()
                            .not_null()
                            .default("PENDING"),
                    )
                    .col(
                        ColumnDef::new(guest_booking::Column::BookingReference)
                            .string_len(100)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(guest_booking::Column::ConfirmTokenHash)
                            .string_len(64)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(guest_booking::Column::ExpiresAt)
                            .date_time()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(guest_booking::Column::PaymentExpiresAt)
                            .date_time()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(guest_booking::Column::Metadata)
                            .json_binary()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(guest_booking::Column::PromotedBookingId)
                            .uuid()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(guest_booking::Column::CreatedAt)
                            .date_time()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP"),
                    )
                    .col(
                        ColumnDef::new(guest_booking::Column::UpdatedAt)
                            .date_time()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP"),
                    )
                    .col(
                        ColumnDef::new(guest_booking::Column::ConfirmedAt)
                            .date_time()
                            .null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_guest_bookings_resource")
                    .table(guest_booking::Entity)
                    .col(guest_booking::Column::ResourceType)
                    .col(guest_booking::Column::ResourceId)
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("idx_guest_bookings_status")
                    .table(guest_booking::Entity)
                    .col(guest_booking::Column::Status)
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("idx_guest_bookings_reference")
                    .table(guest_booking::Entity)
                    .col(guest_booking::Column::BookingReference)
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("idx_guest_bookings_status_expires")
                    .table(guest_booking::Entity)
                    .col(guest_booking::Column::Status)
                    .col(guest_booking::Column::ExpiresAt)
                    .to_owned(),
            )
            .await?;

        // ---- guest_booking_items -------------------------------------------
        manager
            .create_table(
                Table::create()
                    .table(guest_booking_item::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(guest_booking_item::Column::Id)
                            .uuid()
                            .extra("DEFAULT gen_random_uuid()")
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(guest_booking_item::Column::GuestBookingId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(guest_booking_item::Column::ItemType)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(guest_booking_item::Column::ItemId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(guest_booking_item::Column::Price)
                            .float()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(guest_booking_item::Column::Metadata)
                            .json_binary()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(guest_booking_item::Column::CreatedAt)
                            .date_time()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP"),
                    )
                    .col(
                        ColumnDef::new(guest_booking_item::Column::UpdatedAt)
                            .date_time()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP"),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_guest_booking_items_guest_booking")
                            .from(
                                guest_booking_item::Entity,
                                guest_booking_item::Column::GuestBookingId,
                            )
                            .to(guest_booking::Entity, guest_booking::Column::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_guest_booking_items_guest_booking")
                    .table(guest_booking_item::Entity)
                    .col(guest_booking_item::Column::GuestBookingId)
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("idx_guest_booking_items_unit")
                    .table(guest_booking_item::Entity)
                    .col(guest_booking_item::Column::ItemType)
                    .col(guest_booking_item::Column::ItemId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop child first (FK), then parent.
        manager
            .drop_table(Table::drop().table(guest_booking_item::Entity).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(guest_booking::Entity).to_owned())
            .await?;
        Ok(())
    }
}
