use sea_orm_migration::prelude::*;

use features_lookup_entities::lookup_type;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260424_000005_add_tenant_id_to_lookup_type"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(lookup_type::Entity)
                    .add_column(
                        ColumnDef::new(lookup_type::Column::TenantId)
                            .string()
                            .string_len(100)
                            .not_null()
                            .default(""),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_lookup_type_tenant_id")
                    .table(lookup_type::Entity)
                    .col(lookup_type::Column::TenantId)
                    .to_owned(),
            )
            .await?;

        // Make (tenant_id, code) unique instead of just code
        manager
            .drop_index(
                Index::drop()
                    .name("idx_lookup_type_code")
                    .table(lookup_type::Entity)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_lookup_type_tenant_code")
                    .table(lookup_type::Entity)
                    .col(lookup_type::Column::TenantId)
                    .col(lookup_type::Column::Code)
                    .unique()
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(
                Index::drop()
                    .name("idx_lookup_type_tenant_code")
                    .table(lookup_type::Entity)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_lookup_type_code")
                    .table(lookup_type::Entity)
                    .col(lookup_type::Column::Code)
                    .unique()
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .name("idx_lookup_type_tenant_id")
                    .table(lookup_type::Entity)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(lookup_type::Entity)
                    .drop_column(lookup_type::Column::TenantId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}
