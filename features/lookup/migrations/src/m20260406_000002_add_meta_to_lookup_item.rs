use sea_orm_migration::prelude::*;

use features_lookup_entities::lookup_item;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260406_000003_add_meta_to_lookup_item"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let _ = manager
            .alter_table(
                Table::alter()
                    .table(lookup_item::Entity)
                    .add_column(
                        ColumnDef::new(lookup_item::Column::Meta)
                            .json()
                            .not_null()
                            .default("{}"),
                    )
                    .to_owned(),
            )
            .await;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let _ = manager
            .alter_table(
                Table::alter()
                    .table(lookup_item::Entity)
                    .drop_column(lookup_item::Column::Meta)
                    .to_owned(),
            )
            .await;
        Ok(())
    }
}
