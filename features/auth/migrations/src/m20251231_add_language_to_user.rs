use sea_orm_migration::prelude::*;

use features_auth_entities::user;
use shared_shared_data_core::language::Languages;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20251231_add_language_to_user"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let _ = manager
            .alter_table(
                Table::alter()
                    .table(user::Entity)
                    .add_column(
                        ColumnDef::new(user::Column::Language)
                            .string()
                            .not_null()
                            .default(Languages::EnUs.as_str()),
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
                    .table(user::Entity)
                    .drop_column(user::Column::Language)
                    .to_owned(),
            )
            .await
    }
}
