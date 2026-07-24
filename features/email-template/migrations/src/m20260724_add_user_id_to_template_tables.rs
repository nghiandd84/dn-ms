use sea_orm_migration::prelude::*;

use features_email_template_entities::{template_placeholders, template_translations};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Add user_id column to template_placeholders
        manager
            .alter_table(
                Table::alter()
                    .table(template_placeholders::Entity)
                    .add_column(
                        ColumnDef::new(template_placeholders::Column::UserId)
                            .uuid()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        // Add user_id column to template_translations
        manager
            .alter_table(
                Table::alter()
                    .table(template_translations::Entity)
                    .add_column(
                        ColumnDef::new(template_translations::Column::UserId)
                            .uuid()
                            .not_null(),
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
                    .table(template_placeholders::Entity)
                    .drop_column(template_placeholders::Column::UserId)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(template_translations::Entity)
                    .drop_column(template_translations::Column::UserId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}
