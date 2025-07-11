use features_auth_entities::{auth_code, client, client_scope, scope, token, user};
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20250725_upgrade_to_oauth2"
    }
}

impl Migration {
    async fn create_client_table<'a>(
        &'a self,
        manager: &'a SchemaManager<'a>,
    ) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(client::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(client::Column::Id)
                            .uuid()
                            .extra("DEFAULT public.uuid_generate_v4()")
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(client::Column::ClientSecret)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(client::Column::Name)
                            .string_len(128)
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(client::Column::Description)
                            .string_len(512)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(client::Column::RedirectUris)
                            .array(ColumnType::String(StringLen::N(512)))
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(client::Column::AllowedGrants)
                            .array(ColumnType::String(StringLen::N(512)))
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(client::Column::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(client::Column::UpdatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn create_auth_code_table<'a>(
        &'a self,
        manager: &'a SchemaManager<'a>,
    ) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(auth_code::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(auth_code::Column::Id)
                            .uuid()
                            .extra("DEFAULT public.uuid_generate_v4()")
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(auth_code::Column::Code)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(auth_code::Column::ClientId)
                            .uuid()
                            .not_null(),
                    )
                    .col(ColumnDef::new(auth_code::Column::UserId).uuid().not_null())
                    .col(
                        ColumnDef::new(auth_code::Column::RedirectUri)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(auth_code::Column::Scopes)
                            .array(ColumnType::String(StringLen::N(128)))
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(auth_code::Column::ExpiresAt)
                            .timestamp()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(auth_code::Column::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(auth_code::Column::UpdatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(auth_code::Entity, auth_code::Column::ClientId)
                            .to(client::Entity, client::Column::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(auth_code::Entity, auth_code::Column::UserId)
                            .to(user::Entity, user::Column::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn create_token_table<'a>(&'a self, manager: &'a SchemaManager<'a>) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(token::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(token::Column::Id)
                            .uuid()
                            .extra("DEFAULT public.uuid_generate_v4()")
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(token::Column::AccessToken)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(token::Column::RefreshToken)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(token::Column::UserId).uuid().not_null())
                    .col(ColumnDef::new(token::Column::ClientId).uuid().not_null())
                    .col(
                        ColumnDef::new(token::Column::Scopes)
                            .json_binary()
                            .default(Expr::value(serde_json::json!([])))
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(token::Column::AccessTokenExpiresAt)
                            .timestamp()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(token::Column::RefreshTokenExpiresAt)
                            .timestamp()
                            .not_null(),
                    )
                    .col(ColumnDef::new(token::Column::RevokedAt).timestamp().null())
                    .col(
                        ColumnDef::new(token::Column::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(token::Column::UpdatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    // Foreign keys
                    // Assuming you have a user table and client table defined
                    // in your entities.
                    // Adjust the foreign key definitions as per your schema.
                    // For example:
                    // ForeignKey to user table
                    .foreign_key(
                        ForeignKey::create()
                            .from(token::Entity, token::Column::UserId)
                            .to(user::Entity, user::Column::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    // ForeignKey to client table
                    .foreign_key(
                        ForeignKey::create()
                            .from(token::Entity, token::Column::ClientId)
                            .to(client::Entity, client::Column::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn create_scope_table<'a>(&'a self, manager: &'a SchemaManager<'a>) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(scope::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(scope::Column::Id)
                            .uuid()
                            .extra("DEFAULT public.uuid_generate_v4()")
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(scope::Column::Name)
                            .string_len(128)
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(scope::Column::Description)
                            .string_len(512)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(scope::Column::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(scope::Column::UpdatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn create_client_scope_table<'a>(
        &'a self,
        manager: &'a SchemaManager<'a>,
    ) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(client_scope::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(client_scope::Column::Id)
                            .uuid()
                            .extra("DEFAULT public.uuid_generate_v4()")
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(client_scope::Column::ClientId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(client_scope::Column::ScopeId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(client_scope::Column::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(client_scope::Column::UpdatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(client_scope::Entity, client_scope::Column::ClientId)
                            .to(client::Entity, client::Column::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(client_scope::Entity, client_scope::Column::ScopeId)
                            .to(scope::Entity, scope::Column::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Add the `is_active` column to the `users` table
        // with a default value of false and not null constraint.
        let _ = manager
            .alter_table(
                Table::alter()
                    .table(user::Entity)
                    .add_column(
                        ColumnDef::new(user::Column::IsActive)
                            .boolean()
                            .default(false)
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await;

        self.create_client_table(manager).await?;
        self.create_auth_code_table(manager).await?;
        self.create_token_table(manager).await?;
        self.create_scope_table(manager).await?;
        self.create_client_scope_table(manager).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Remove the `is_active` column from the `users` table
        // This is a destructive operation, so ensure you have backups if necessary.
        let _ = manager
            .alter_table(
                Table::alter()
                    .table(user::Entity)
                    .drop_column(user::Column::IsActive)
                    .to_owned(),
            )
            .await;

        // Drop the tables in reverse order of creation
        let _ = manager
            .drop_table(Table::drop().table(token::Entity).to_owned())
            .await;

        let _ = manager
            .drop_table(Table::drop().table(auth_code::Entity).to_owned())
            .await;

        let _ = manager
            .drop_table(Table::drop().table(client::Entity).to_owned())
            .await;

        let _ = manager
            .drop_table(Table::drop().table(client_scope::Entity).to_owned())
            .await;

        let _ = manager
            .drop_table(Table::drop().table(scope::Entity).to_owned())
            .await;

        Ok(())
    }
}
