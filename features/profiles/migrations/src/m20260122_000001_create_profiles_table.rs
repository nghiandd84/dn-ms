use features_profiles_entities::profile;
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260122_000001_create_profiles_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(profile::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(profile::Column::Id)
                            .uuid()
                            .extra("DEFAULT public.uuid_generate_v4()")
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(profile::Column::UserId)
                            .uuid()
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(profile::Column::FirstName)
                            .string()
                            .string_len(100)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(profile::Column::LastName)
                            .string()
                            .string_len(100)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(profile::Column::Bio)
                            .text()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(profile::Column::AvatarUrl)
                            .string()
                            .string_len(500)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(profile::Column::Location)
                            .string()
                            .string_len(255)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(profile::Column::CreatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(profile::Column::UpdatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(profile::Entity).to_owned())
            .await
    }
}
