use features_auth_entities::authentication;
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20251031_create_authentication_requests"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(authentication::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(authentication::Column::Id)
                            .uuid()
                            .extra("DEFAULT public.uuid_generate_v4()")
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(authentication::Column::ClientId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(authentication::Column::Scopes)
                            .array(ColumnType::String(StringLen::N(128)))
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(authentication::Column::ResponseType)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(authentication::Column::RedirectUri)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(authentication::Column::State)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(authentication::Column::ExpiresAt)
                            .timestamp()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(authentication::Column::CreatedAt)
                            .timestamp()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(authentication::Column::UpdatedAt)
                            .timestamp()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(authentication::Entity).to_owned())
            .await
    }
}
