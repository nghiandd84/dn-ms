use sea_orm_migration::prelude::*;

use features_email_template_entities::email_templates;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20251229_add_key_to_email_templates"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let _ = manager
            .alter_table(
                Table::alter()
                    .table(email_templates::Entity)
                    .add_column_if_not_exists(
                        ColumnDef::new(email_templates::Column::Key)
                            .string_len(255)
                            .unique_key()
                            .default("")
                            .not_null(),
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
                    .table(email_templates::Entity)
                    .drop_column(email_templates::Column::Key)
                    .to_owned(),
            )
            .await;
        Ok(())
    }
}
