use features_profiles_entities::{profile, user_preference};
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260122_000002_create_user_preferences_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(user_preference::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(user_preference::Column::Id)
                            .uuid()
                            .extra("DEFAULT public.uuid_generate_v4()")
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(user_preference::Column::ProfileId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(user_preference::Column::Language)
                            .string()
                            .string_len(10)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(user_preference::Column::Theme)
                            .string()
                            .string_len(20)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(user_preference::Column::NotificationsEnabled)
                            .boolean()
                            .default(true)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(user_preference::Column::CreatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(user_preference::Column::UpdatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_user_preferences_profile_id")
                            .from(user_preference::Entity, user_preference::Column::ProfileId)
                            .to(profile::Entity, profile::Column::Id)
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(user_preference::Entity).to_owned())
            .await
    }
}
