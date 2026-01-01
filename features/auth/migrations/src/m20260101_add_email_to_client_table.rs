use sea_orm_migration::prelude::*;

use features_auth_entities::client;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260101_add_email_to_client_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let _ = manager
            .alter_table(
                Table::alter()
                    .table(client::Entity)
                    .add_column(
                        ColumnDef::new(client::Column::Email)
                            .string()
                            .not_null()
                            .default(""),
                    )
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(client::Entity)
                    .drop_column(client::Column::Email)
                    .to_owned(),
            )
            .await
    }
}
