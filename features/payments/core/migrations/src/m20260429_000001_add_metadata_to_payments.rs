use sea_orm_migration::prelude::*;

use features_payments_core_entities::payment;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260429_000001_add_metadata_to_payments"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(payment::Entity)
                    .add_column(ColumnDef::new(payment::Column::Metadata).json().null())
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(payment::Entity)
                    .drop_column(payment::Column::Metadata)
                    .to_owned(),
            )
            .await?;
        Ok(())
    }
}
