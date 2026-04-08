use features_profiles_entities::{profile, social_link};
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260122_000003_create_social_links_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(social_link::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(social_link::Column::Id)
                            .uuid()
                            .extra("DEFAULT public.uuid_generate_v4()")
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(social_link::Column::ProfileId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(social_link::Column::Platform)
                            .string()
                            .string_len(50)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(social_link::Column::Url)
                            .string()
                            .string_len(500)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(social_link::Column::CreatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(social_link::Column::UpdatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_social_links_profile_id")
                            .from(social_link::Entity, social_link::Column::ProfileId)
                            .to(profile::Entity, profile::Column::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(social_link::Entity).to_owned())
            .await
    }
}
