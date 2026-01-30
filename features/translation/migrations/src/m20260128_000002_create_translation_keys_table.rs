use sea_orm_migration::prelude::*;

use features_translation_entities::project;
use features_translation_entities::translation_key;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260128_000002_create_translation_keys_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(translation_key::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(translation_key::Column::Id)
                            .uuid()
                            .extra("DEFAULT public.uuid_generate_v4()")
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(translation_key::Column::ProjectId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(translation_key::Column::UserId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(translation_key::Column::KeyName)
                            .string()
                            .string_len(255)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(translation_key::Column::Description)
                            .text()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(translation_key::Column::CreatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(translation_key::Column::UpdatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_translation_keys_project_id")
                            .from(translation_key::Entity, translation_key::Column::ProjectId)
                            .to(project::Entity, project::Column::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .index(
                        Index::create()
                            .name("idx_translation_keys_unique")
                            .table(translation_key::Entity)
                            .col(translation_key::Column::ProjectId)
                            .col(translation_key::Column::KeyName)
                            .unique(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(translation_key::Entity).to_owned())
            .await
    }
}
