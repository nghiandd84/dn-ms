// m20250801_162140_create_email_tables.rs

use sea_orm_migration::prelude::*;

use features_email_template_entities::{
    email_templates, template_placeholders, template_translations,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create the 'email_templates' table
        manager
            .create_table(
                Table::create()
                    .table(email_templates::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(email_templates::Column::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(email_templates::Column::Name)
                            .string_len(255)
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(email_templates::Column::Description).text())
                    .col(
                        ColumnDef::new(email_templates::Column::IsActive)
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .col(
                        ColumnDef::new(email_templates::Column::UserId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(email_templates::Column::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(email_templates::Column::UpdatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        // Create the 'template_translations' table
        manager
            .create_table(
                Table::create()
                    .table(template_translations::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(template_translations::Column::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(template_translations::Column::TemplateId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(template_translations::Column::LanguageCode)
                            .string_len(10)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(template_translations::Column::Subject)
                            .string_len(255)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(template_translations::Column::Body)
                            .text()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(template_translations::Column::VersionName)
                            .string_len(50)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(template_translations::Column::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(template_translations::Column::UpdatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-template_translations-template_id")
                            .from(
                                template_translations::Entity,
                                template_placeholders::Column::TemplateId,
                            )
                            .to(email_templates::Entity, email_templates::Column::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Add a unique index to prevent duplicate translations
        manager
            .create_index(
                Index::create()
                    .name("idx-template_translations-unique")
                    .table(template_translations::Entity)
                    .col(template_translations::Column::TemplateId)
                    .col(template_translations::Column::LanguageCode)
                    .col(template_translations::Column::VersionName)
                    .unique()
                    .to_owned(),
            )
            .await?;

        // Create the 'template_placeholders' table
        manager
            .create_table(
                Table::create()
                    .table(template_placeholders::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(template_placeholders::Column::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(template_placeholders::Column::TemplateId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(template_placeholders::Column::PlaceholderKey)
                            .string_len(100)
                            .not_null(),
                    )
                    .col(ColumnDef::new(template_placeholders::Column::Description).text())
                    .col(ColumnDef::new(template_placeholders::Column::ExampleValue).text())
                    .col(
                        ColumnDef::new(template_placeholders::Column::IsRequired)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(template_placeholders::Column::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(template_placeholders::Column::UpdatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-template_placeholders-template_id")
                            .from(
                                template_placeholders::Entity,
                                template_placeholders::Column::TemplateId,
                            )
                            .to(email_templates::Entity, email_templates::Column::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(template_translations::Entity)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .table(template_translations::Entity)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().table(email_templates::Entity).to_owned())
            .await?;

        Ok(())
    }
}
