use sea_orm_migration::prelude::*;

use features_lookup_entities::{lookup_item, lookup_item_translation, lookup_type};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260403_000001_create_lookup_tables"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let _create_lookup_type_table = manager
            .create_table(
                Table::create()
                    .table(lookup_type::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(lookup_type::Column::Id)
                            .uuid()
                            .extra("DEFAULT public.uuid_generate_v4()")
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(lookup_type::Column::Code)
                            .string()
                            .string_len(50)
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(lookup_type::Column::Name)
                            .string()
                            .string_len(100)
                            .not_null(),
                    )
                    .col(ColumnDef::new(lookup_type::Column::Description).text())
                    .col(
                        ColumnDef::new(lookup_type::Column::IsActive)
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .col(
                        ColumnDef::new(lookup_type::Column::CreatedAt)
                            .date_time()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP"),
                    )
                    .col(
                        ColumnDef::new(lookup_type::Column::UpdatedAt)
                            .date_time()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP"),
                    )
                    .to_owned(),
            )
            .await;

        let _create_lookup_item_table = manager
            .create_table(
                Table::create()
                    .table(lookup_item::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(lookup_item::Column::Id)
                            .uuid()
                            .extra("DEFAULT public.uuid_generate_v4()")
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(lookup_item::Column::LookupTypeId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(lookup_item::Column::Code)
                            .string()
                            .string_len(50)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(lookup_item::Column::Name)
                            .string()
                            .string_len(200)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(lookup_item::Column::Url)
                            .string()
                            .string_len(500),
                    )
                    .col(
                        ColumnDef::new(lookup_item::Column::QueryParamOne)
                            .string()
                            .string_len(200),
                    )
                    .col(
                        ColumnDef::new(lookup_item::Column::QueryParamTwo)
                            .string()
                            .string_len(200),
                    )
                    .col(
                        ColumnDef::new(lookup_item::Column::Tenants)
                            .array(ColumnType::String(StringLen::N(50)))
                            .not_null()
                            .default("ARRAY[]::VARCHAR[]"),
                    )
                    .col(
                        ColumnDef::new(lookup_item::Column::IsActive)
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .col(
                        ColumnDef::new(lookup_item::Column::SortOrder)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(lookup_item::Column::CreatedAt)
                            .date_time()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP"),
                    )
                    .col(
                        ColumnDef::new(lookup_item::Column::UpdatedAt)
                            .date_time()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP"),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-lookup-item-type")
                            .from(lookup_item::Entity, lookup_item::Column::LookupTypeId)
                            .to(lookup_type::Entity, lookup_type::Column::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await;

        let _create_lookup_item_translation_table = manager
            .create_table(
                Table::create()
                    .table(lookup_item_translation::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(lookup_item_translation::Column::Id)
                            .uuid()
                            .extra("DEFAULT public.uuid_generate_v4()")
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(lookup_item_translation::Column::LookupItemId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(lookup_item_translation::Column::Locale)
                            .string()
                            .string_len(10)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(lookup_item_translation::Column::Name)
                            .string()
                            .string_len(200)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(lookup_item_translation::Column::CreatedAt)
                            .date_time()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP"),
                    )
                    .col(
                        ColumnDef::new(lookup_item_translation::Column::UpdatedAt)
                            .date_time()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP"),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-lookup-translation-item")
                            .from(
                                lookup_item_translation::Entity,
                                lookup_item_translation::Column::LookupItemId,
                            )
                            .to(lookup_item::Entity, lookup_item::Column::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await;

        // Indexes
        let _idx_lookup_type_code = manager
            .create_index(
                Index::create()
                    .name("idx_lookup_type_code")
                    .table(lookup_type::Entity)
                    .col(lookup_type::Column::Code)
                    .unique()
                    .to_owned(),
            )
            .await;

        let _idx_lookup_item_type = manager
            .create_index(
                Index::create()
                    .name("idx_lookup_item_type")
                    .table(lookup_item::Entity)
                    .col(lookup_item::Column::LookupTypeId)
                    .to_owned(),
            )
            .await;

        let _idx_lookup_item_type_code = manager
            .create_index(
                Index::create()
                    .name("idx_lookup_item_type_code")
                    .table(lookup_item::Entity)
                    .col(lookup_item::Column::LookupTypeId)
                    .col(lookup_item::Column::Code)
                    .to_owned(),
            )
            .await;

        let _idx_lookup_translation_item_locale = manager
            .create_index(
                Index::create()
                    .name("idx_lookup_translation_item_locale")
                    .table(lookup_item_translation::Entity)
                    .col(lookup_item_translation::Column::LookupItemId)
                    .col(lookup_item_translation::Column::Locale)
                    .unique()
                    .to_owned(),
            )
            .await;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop indexes
        let _drop_idx_lookup_type_code = manager
            .drop_index(
                Index::drop()
                    .name("idx_lookup_type_code")
                    .table(lookup_type::Entity)
                    .to_owned(),
            )
            .await;

        let _drop_idx_lookup_item_type = manager
            .drop_index(
                Index::drop()
                    .name("idx_lookup_item_type")
                    .table(lookup_item::Entity)
                    .to_owned(),
            )
            .await;

        let _drop_idx_lookup_item_type_code = manager
            .drop_index(
                Index::drop()
                    .name("idx_lookup_item_type_code")
                    .table(lookup_item::Entity)
                    .to_owned(),
            )
            .await;

        let _drop_idx_lookup_translation_item_locale = manager
            .drop_index(
                Index::drop()
                    .name("idx_lookup_translation_item_locale")
                    .table(lookup_item_translation::Entity)
                    .to_owned(),
            )
            .await;

        // Drop tables in reverse dependency order
        let _drop_lookup_item_translation_table = manager
            .drop_table(
                Table::drop()
                    .table(lookup_item_translation::Entity)
                    .to_owned(),
            )
            .await;

        let _drop_lookup_item_table = manager
            .drop_table(Table::drop().table(lookup_item::Entity).to_owned())
            .await;

        let _drop_lookup_type_table = manager
            .drop_table(Table::drop().table(lookup_type::Entity).to_owned())
            .await;

        Ok(())
    }
}
