use features_translation_entities::key_tag;
use features_translation_entities::tag;
use features_translation_entities::translation_key;
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260128_000004_create_key_tags_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(key_tag::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(key_tag::Column::Id)
                            .uuid()
                            .extra("DEFAULT gen_random_uuid()")
                            .not_null(),
                    )
                    .col(ColumnDef::new(key_tag::Column::KeyId).uuid().not_null())
                    .col(ColumnDef::new(key_tag::Column::TagId).uuid().not_null())
                    .col(
                        ColumnDef::new(key_tag::Column::CreatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(key_tag::Column::UpdatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .primary_key(
                        Index::create()
                            .name("pk_key_tags")
                            .col(key_tag::Column::KeyId)
                            .col(key_tag::Column::TagId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_key_tags_key_id")
                            .from(key_tag::Entity, key_tag::Column::KeyId)
                            .to(translation_key::Entity, translation_key::Column::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_key_tags_tag_id")
                            .from(key_tag::Entity, key_tag::Column::TagId)
                            .to(tag::Entity, tag::Column::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(key_tag::Entity).to_owned())
            .await
    }
}
