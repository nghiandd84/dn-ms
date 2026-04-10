use features_translation_entities::translation_key;
use features_translation_entities::translation_version;
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260128_000005_create_translation_versions_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(translation_version::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(translation_version::Column::Id)
                            .uuid()
                            .extra("DEFAULT gen_random_uuid()")
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(translation_version::Column::KeyId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(translation_version::Column::Locale)
                            .string()
                            .string_len(10)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(translation_version::Column::Content)
                            .text()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(translation_version::Column::VersionNumber)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(translation_version::Column::Status)
                            .string()
                            .string_len(20)
                            .not_null()
                            .default("draft"),
                    )
                    .col(
                        ColumnDef::new(translation_version::Column::CreatedBy)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(translation_version::Column::CreatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(translation_version::Column::UpdatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_translation_key_id")
                            .from(
                                translation_version::Entity,
                                translation_version::Column::KeyId,
                            )
                            .to(translation_key::Entity, translation_key::Column::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .index(
                        Index::create()
                            .name("idx_translation_latest")
                            .table(translation_version::Entity)
                            .col(translation_version::Column::KeyId)
                            .col(translation_version::Column::Locale)
                            .col(translation_version::Column::Status)
                            .col(translation_version::Column::VersionNumber)
                            .unique(),
                    )
                    .index(
                        Index::create()
                            .name("idx_translation_versions_unique")
                            .table(translation_version::Entity)
                            .col(translation_version::Column::KeyId)
                            .col(translation_version::Column::Locale)
                            .col(translation_version::Column::VersionNumber)
                            .unique(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(translation_version::Entity).to_owned())
            .await
    }
}
