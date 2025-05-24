use features_auth_entities::role;
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20250424_create_role"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(role::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(role::Column::Id)
                            .uuid()
                            .extra("DEFAULT public.uuid_generate_v4()")
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(role::Column::Name)
                            .string()
                            .string_len(50)
                            .unique_key()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(role::Column::Description)
                            .string()
                            .string_len(250)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(role::Column::CreatedAt)
                            .timestamp()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(role::Column::UpdatedAt)
                            .timestamp()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(role::Entity).to_owned())
            .await
    }
}
