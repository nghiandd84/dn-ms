use features_auth_entities::active_code;
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20251225_add_active_code_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(active_code::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(active_code::Column::Id)
                            .uuid()
                            .extra("DEFAULT gen_random_uuid()")
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(active_code::Column::Code)
                            .string()
                            .string_len(50)
                            .unique_key()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(active_code::Column::IsUsed)
                            .boolean()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(active_code::Column::ExpirationTime)
                            .timestamp()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(active_code::Column::CreatedAt)
                            .timestamp()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(active_code::Column::UpdatedAt)
                            .timestamp()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(active_code::Entity).to_owned())
            .await
    }
}
