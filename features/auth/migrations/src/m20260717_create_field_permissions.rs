use sea_orm_migration::prelude::*;

use features_auth_entities::field_permission;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260717_create_field_permissions"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(field_permission::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(field_permission::Column::Id)
                            .uuid()
                            .extra("DEFAULT gen_random_uuid()")
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(field_permission::Column::RoleId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(field_permission::Column::Resource)
                            .string()
                            .string_len(255)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(field_permission::Column::Action)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(field_permission::Column::Fields)
                            .array(ColumnType::Text)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(field_permission::Column::CreatedAt)
                            .timestamp()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(field_permission::Column::UpdatedAt)
                            .timestamp()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(field_permission::Entity, field_permission::Column::RoleId)
                            .to(
                                features_auth_entities::role::Entity,
                                features_auth_entities::role::Column::Id,
                            )
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Unique constraint: one entry per role + resource + action
        manager
            .create_index(
                Index::create()
                    .name("idx_field_permissions_role_resource_action")
                    .table(field_permission::Entity)
                    .col(field_permission::Column::RoleId)
                    .col(field_permission::Column::Resource)
                    .col(field_permission::Column::Action)
                    .unique()
                    .to_owned(),
            )
            .await?;

        // Index on resource for service-scoped lookups
        manager
            .create_index(
                Index::create()
                    .name("idx_field_permissions_resource")
                    .table(field_permission::Entity)
                    .col(field_permission::Column::Resource)
                    .to_owned(),
            )
            .await?;

        // Index on role_id for role-scoped lookups
        manager
            .create_index(
                Index::create()
                    .name("idx_field_permissions_role_id")
                    .table(field_permission::Entity)
                    .col(field_permission::Column::RoleId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(field_permission::Entity).to_owned())
            .await?;
        Ok(())
    }
}
