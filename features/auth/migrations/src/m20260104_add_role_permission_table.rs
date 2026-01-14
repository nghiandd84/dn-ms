use sea_orm_migration::prelude::*;

use features_auth_entities::permission;
use features_auth_entities::role_permission;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260104_add_role_permission_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let _ = manager
            .create_table(
                Table::create()
                    .table(permission::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(permission::Column::Id)
                            .uuid()
                            .extra("DEFAULT public.uuid_generate_v4()")
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(permission::Column::Resource)
                            .string()
                            .string_len(1024)
                            .unique_key()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(permission::Column::Description)
                            .string()
                            .string_len(250)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(permission::Column::CreatedAt)
                            .timestamp()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(permission::Column::UpdatedAt)
                            .timestamp()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;
        let _ = manager
            .create_table(
                Table::create()
                    .table(role_permission::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(role_permission::Column::Id)
                            .uuid()
                            .extra("DEFAULT public.uuid_generate_v4()")
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(role_permission::Column::RoleId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(role_permission::Column::PermissionId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(role_permission::Column::CreatedAt)
                            .timestamp()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(role_permission::Column::UpdatedAt)
                            .timestamp()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let _drop_permission = manager
            .drop_table(Table::drop().table(permission::Entity).to_owned())
            .await;
        let _drop_role_permission = manager
            .drop_table(Table::drop().table(role_permission::Entity).to_owned())
            .await;
        Ok(())
    }
}
