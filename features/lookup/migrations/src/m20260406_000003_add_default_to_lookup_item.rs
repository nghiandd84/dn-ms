use sea_orm_migration::prelude::*;

use features_lookup_entities::lookup_item;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260406_000003_add_default_to_lookup_item"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let _ = manager
            .alter_table(
                Table::alter()
                    .table(lookup_item::Entity)
                    .modify_column(
                        ColumnDef::new(lookup_item::Column::Url)
                            .string()
                            .not_null()
                            .default(""),
                    )
                    .to_owned(),
            )
            .await;
        let _ = manager
            .alter_table(
                Table::alter()
                    .table(lookup_item::Entity)
                    .modify_column(
                        ColumnDef::new(lookup_item::Column::QueryParamOne)
                            .string()
                            .not_null()
                            .default(""),
                    )
                    .to_owned(),
            )
            .await;
        let _ = manager
            .alter_table(
                Table::alter()
                    .table(lookup_item::Entity)
                    .modify_column(
                        ColumnDef::new(lookup_item::Column::QueryParamTwo)
                            .string()
                            .not_null()
                            .default(""),
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
                    .modify_column(ColumnDef::new(lookup_item::Column::Url).string().not_null())
                    .to_owned(),
            )
            .await;

        let _ = manager
            .alter_table(
                Table::alter()
                    .table(lookup_item::Entity)
                    .modify_column(
                        ColumnDef::new(lookup_item::Column::QueryParamOne)
                            .string()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await;

        let _ = manager
            .alter_table(
                Table::alter()
                    .table(lookup_item::Entity)
                    .modify_column(
                        ColumnDef::new(lookup_item::Column::QueryParamTwo)
                            .string()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await;

        Ok(())
    }
}
