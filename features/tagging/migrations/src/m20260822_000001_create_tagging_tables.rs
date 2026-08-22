use sea_orm_migration::prelude::*;

use features_tagging_entities::{entity_tag, tag, tag_group};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260822_000001_create_tagging_tables"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create tag_groups table
        manager
            .create_table(
                Table::create()
                    .table(tag_group::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(tag_group::Column::Id)
                            .uuid()
                            .extra("DEFAULT gen_random_uuid()")
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(tag_group::Column::TenantId)
                            .string()
                            .string_len(50)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(tag_group::Column::Code)
                            .string()
                            .string_len(50)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(tag_group::Column::Name)
                            .string()
                            .string_len(100)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(tag_group::Column::Description)
                            .text()
                            .not_null()
                            .default(""),
                    )
                    .col(ColumnDef::new(tag_group::Column::ParentId).uuid().null())
                    .col(
                        ColumnDef::new(tag_group::Column::IsActive)
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .col(
                        ColumnDef::new(tag_group::Column::SortOrder)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(tag_group::Column::CreatedAt)
                            .date_time()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP"),
                    )
                    .col(
                        ColumnDef::new(tag_group::Column::UpdatedAt)
                            .date_time()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP"),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_tag_groups_parent_id")
                            .from(tag_group::Entity, tag_group::Column::ParentId)
                            .to(tag_group::Entity, tag_group::Column::Id)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .to_owned(),
            )
            .await?;

        // Unique constraint on (tenant_id, code)
        manager
            .create_index(
                Index::create()
                    .name("idx_tag_groups_tenant_code")
                    .table(tag_group::Entity)
                    .col(tag_group::Column::TenantId)
                    .col(tag_group::Column::Code)
                    .unique()
                    .to_owned(),
            )
            .await?;

        // Index on tenant_id for filtering
        manager
            .create_index(
                Index::create()
                    .name("idx_tag_groups_tenant_id")
                    .table(tag_group::Entity)
                    .col(tag_group::Column::TenantId)
                    .to_owned(),
            )
            .await?;

        // Create tags table
        manager
            .create_table(
                Table::create()
                    .table(tag::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(tag::Column::Id)
                            .uuid()
                            .extra("DEFAULT gen_random_uuid()")
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(tag::Column::TenantId)
                            .string()
                            .string_len(50)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(tag::Column::TagGroupId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(tag::Column::Name)
                            .string()
                            .string_len(100)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(tag::Column::Slug)
                            .string()
                            .string_len(120)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(tag::Column::Color)
                            .string()
                            .string_len(7)
                            .not_null()
                            .default("#000000"),
                    )
                    .col(
                        ColumnDef::new(tag::Column::Description)
                            .text()
                            .not_null()
                            .default(""),
                    )
                    .col(ColumnDef::new(tag::Column::AliasOf).uuid().null())
                    .col(
                        ColumnDef::new(tag::Column::IsActive)
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .col(
                        ColumnDef::new(tag::Column::SortOrder)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(tag::Column::UsageCount)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(tag::Column::CreatedAt)
                            .date_time()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP"),
                    )
                    .col(
                        ColumnDef::new(tag::Column::UpdatedAt)
                            .date_time()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP"),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_tags_tag_group_id")
                            .from(tag::Entity, tag::Column::TagGroupId)
                            .to(tag_group::Entity, tag_group::Column::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_tags_alias_of")
                            .from(tag::Entity, tag::Column::AliasOf)
                            .to(tag::Entity, tag::Column::Id)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .to_owned(),
            )
            .await?;

        // Unique constraint on (tenant_id, slug)
        manager
            .create_index(
                Index::create()
                    .name("idx_tags_tenant_slug")
                    .table(tag::Entity)
                    .col(tag::Column::TenantId)
                    .col(tag::Column::Slug)
                    .unique()
                    .to_owned(),
            )
            .await?;

        // Index on (tenant_id, tag_group_id)
        manager
            .create_index(
                Index::create()
                    .name("idx_tags_tenant_group")
                    .table(tag::Entity)
                    .col(tag::Column::TenantId)
                    .col(tag::Column::TagGroupId)
                    .to_owned(),
            )
            .await?;

        // Index on tenant_id + name for search/autocomplete
        manager
            .create_index(
                Index::create()
                    .name("idx_tags_tenant_name")
                    .table(tag::Entity)
                    .col(tag::Column::TenantId)
                    .col(tag::Column::Name)
                    .to_owned(),
            )
            .await?;

        // Create entity_tags table
        manager
            .create_table(
                Table::create()
                    .table(entity_tag::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(entity_tag::Column::Id)
                            .uuid()
                            .extra("DEFAULT gen_random_uuid()")
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(entity_tag::Column::TagId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(entity_tag::Column::EntityType)
                            .string()
                            .string_len(50)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(entity_tag::Column::EntityId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(entity_tag::Column::TenantId)
                            .string()
                            .string_len(50)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(entity_tag::Column::TaggedBy)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(entity_tag::Column::CreatedAt)
                            .date_time()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP"),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_entity_tags_tag_id")
                            .from(entity_tag::Entity, entity_tag::Column::TagId)
                            .to(tag::Entity, tag::Column::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Unique constraint on (tag_id, entity_type, entity_id) — prevent duplicate tagging
        manager
            .create_index(
                Index::create()
                    .name("idx_entity_tags_unique")
                    .table(entity_tag::Entity)
                    .col(entity_tag::Column::TagId)
                    .col(entity_tag::Column::EntityType)
                    .col(entity_tag::Column::EntityId)
                    .unique()
                    .to_owned(),
            )
            .await?;

        // Index on (entity_type, entity_id) for "get tags for entity" queries
        manager
            .create_index(
                Index::create()
                    .name("idx_entity_tags_entity")
                    .table(entity_tag::Entity)
                    .col(entity_tag::Column::EntityType)
                    .col(entity_tag::Column::EntityId)
                    .to_owned(),
            )
            .await?;

        // Index on (tenant_id, entity_type) for tenant-scoped queries
        manager
            .create_index(
                Index::create()
                    .name("idx_entity_tags_tenant_type")
                    .table(entity_tag::Entity)
                    .col(entity_tag::Column::TenantId)
                    .col(entity_tag::Column::EntityType)
                    .to_owned(),
            )
            .await?;

        // Index on tag_id for "get entities for tag" queries
        manager
            .create_index(
                Index::create()
                    .name("idx_entity_tags_tag_id")
                    .table(entity_tag::Entity)
                    .col(entity_tag::Column::TagId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(entity_tag::Entity).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(tag::Entity).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(tag_group::Entity).to_owned())
            .await?;
        Ok(())
    }
}
