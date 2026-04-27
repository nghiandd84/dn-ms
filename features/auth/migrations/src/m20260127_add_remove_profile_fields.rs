use sea_orm_migration::prelude::*;

use features_auth_entities::user;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260127_add_remove_profile_fields"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        if manager.has_column("users", "first_name").await? {
            manager
                .alter_table(
                    Table::alter()
                        .table(Alias::new("users"))
                        .drop_column(Alias::new("first_name"))
                        .to_owned(),
                )
                .await?;
        }

        if manager.has_column("users", "last_name").await? {
            manager
                .alter_table(
                    Table::alter()
                        .table(Alias::new("users"))
                        .drop_column(Alias::new("last_name"))
                        .to_owned(),
                )
                .await?;
        }
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let _revert_profile = manager
            .alter_table(
                Table::alter()
                    .table(user::Entity)
                    .add_column_if_not_exists(
                        ColumnDef::new("first_name").string().default("").not_null(),
                    )
                    .add_column_if_not_exists(
                        ColumnDef::new("last_name").string().default("").not_null(),
                    )
                    .to_owned(),
            )
            .await;

        Ok(())
    }
}
