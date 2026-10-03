use sea_orm_migration::prelude::*;

use features_booking_entities::booking_item;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20261003_000001_alter_booking_item_price_to_real"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    /// Align `booking_items.price` with the SeaORM entity (`f32` / FLOAT4).
    ///
    /// The column had drifted to `NUMERIC`, which SeaORM cannot decode into
    /// `f32`, causing guest-booking promotion (and any booking item read/insert
    /// with RETURNING) to fail with a ColumnDecode error. This converts it to
    /// `real`, matching the other money columns in this schema.
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(booking_item::Entity)
                    .modify_column(ColumnDef::new(booking_item::Column::Price).float().not_null())
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    /// Revert `booking_items.price` back to `NUMERIC(16, 4)`.
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(booking_item::Entity)
                    .modify_column(
                        ColumnDef::new(booking_item::Column::Price)
                            .decimal_len(16, 4)
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;
        Ok(())
    }
}
