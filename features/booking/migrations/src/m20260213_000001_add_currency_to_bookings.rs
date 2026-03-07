use sea_orm_migration::prelude::*;

use features_booking_entities::booking;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260213_000001_add_currency_to_bookings"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let _add_currency_to_bookings = manager
            .alter_table(
                Table::alter()
                    .table(booking::Entity)
                    .add_column(
                        ColumnDef::new(booking::Column::Currency)
                            .string()
                            .not_null()
                            .default("USD"),
                    )
                    .to_owned(),
            )
            .await;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let _drop_currency_from_bookings = manager
            .alter_table(
                Table::alter()
                    .table(booking::Entity)
                    .drop_column(booking::Column::Currency)
                    .to_owned(),
            )
            .await;

        Ok(())
    }
}
