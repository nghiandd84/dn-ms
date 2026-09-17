use sea_orm_migration::prelude::*;

use features_booking_entities::{guest_booking, guest_booking_history};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260218_000001_create_guest_booking_history"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(guest_booking_history::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(guest_booking_history::Column::Id)
                            .uuid()
                            .extra("DEFAULT gen_random_uuid()")
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(guest_booking_history::Column::GuestBookingId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(guest_booking_history::Column::EventType)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(guest_booking_history::Column::FromStatus)
                            .string()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(guest_booking_history::Column::ToStatus)
                            .string()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(guest_booking_history::Column::ActorId)
                            .uuid()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(guest_booking_history::Column::Note)
                            .string_len(500)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(guest_booking_history::Column::Metadata)
                            .json_binary()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(guest_booking_history::Column::CreatedAt)
                            .date_time()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP"),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_guest_booking_history_guest_booking")
                            .from(
                                guest_booking_history::Entity,
                                guest_booking_history::Column::GuestBookingId,
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
                    .name("idx_guest_booking_history_guest_booking")
                    .table(guest_booking_history::Entity)
                    .col(guest_booking_history::Column::GuestBookingId)
                    .col(guest_booking_history::Column::CreatedAt)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(guest_booking_history::Entity)
                    .to_owned(),
            )
            .await?;
        Ok(())
    }
}
