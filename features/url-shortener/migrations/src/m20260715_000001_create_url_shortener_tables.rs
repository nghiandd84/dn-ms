use sea_orm_migration::prelude::*;

use features_url_shortener_entities::{api_key, shortened_url, url_click};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260715_000001_create_url_shortener_tables"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create shortened_urls table
        manager
            .create_table(
                Table::create()
                    .table(shortened_url::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(shortened_url::Column::Id)
                            .uuid()
                            .extra("DEFAULT gen_random_uuid()")
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(shortened_url::Column::UserId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(shortened_url::Column::OriginalUrl)
                            .text()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(shortened_url::Column::ShortCode)
                            .string()
                            .string_len(30)
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(shortened_url::Column::Title)
                            .string()
                            .string_len(255),
                    )
                    .col(
                        ColumnDef::new(shortened_url::Column::IsActive)
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .col(ColumnDef::new(shortened_url::Column::ExpiresAt).date_time())
                    .col(
                        ColumnDef::new(shortened_url::Column::ClickCount)
                            .big_integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(shortened_url::Column::CreatedAt)
                            .date_time()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP"),
                    )
                    .col(
                        ColumnDef::new(shortened_url::Column::UpdatedAt)
                            .date_time()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP"),
                    )
                    .to_owned(),
            )
            .await?;

        // Create index on shortened_urls.user_id
        manager
            .create_index(
                Index::create()
                    .name("idx_shortened_urls_user_id")
                    .table(shortened_url::Entity)
                    .col(shortened_url::Column::UserId)
                    .to_owned(),
            )
            .await?;

        // Create url_clicks table
        manager
            .create_table(
                Table::create()
                    .table(url_click::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(url_click::Column::Id)
                            .uuid()
                            .extra("DEFAULT gen_random_uuid()")
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(url_click::Column::UrlId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(url_click::Column::IpAddress)
                            .string()
                            .string_len(45),
                    )
                    .col(ColumnDef::new(url_click::Column::UserAgent).text())
                    .col(ColumnDef::new(url_click::Column::Referrer).text())
                    .col(
                        ColumnDef::new(url_click::Column::Country)
                            .string()
                            .string_len(2),
                    )
                    .col(
                        ColumnDef::new(url_click::Column::ClickedAt)
                            .date_time()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP"),
                    )
                    .col(
                        ColumnDef::new(url_click::Column::CreatedAt)
                            .date_time()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP"),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_url_clicks_url_id")
                            .from(url_click::Entity, url_click::Column::UrlId)
                            .to(shortened_url::Entity, shortened_url::Column::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Create composite index on url_clicks(url_id, clicked_at)
        manager
            .create_index(
                Index::create()
                    .name("idx_url_clicks_url_id_clicked_at")
                    .table(url_click::Entity)
                    .col(url_click::Column::UrlId)
                    .col(url_click::Column::ClickedAt)
                    .to_owned(),
            )
            .await?;

        // Create api_keys table
        manager
            .create_table(
                Table::create()
                    .table(api_key::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(api_key::Column::Id)
                            .uuid()
                            .extra("DEFAULT gen_random_uuid()")
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(api_key::Column::UserId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(api_key::Column::KeyHash)
                            .string()
                            .string_len(64)
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(api_key::Column::Name)
                            .string()
                            .string_len(100)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(api_key::Column::IsActive)
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .col(ColumnDef::new(api_key::Column::LastUsedAt).date_time())
                    .col(
                        ColumnDef::new(api_key::Column::CreatedAt)
                            .date_time()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP"),
                    )
                    .to_owned(),
            )
            .await?;

        // Create index on api_keys.user_id
        manager
            .create_index(
                Index::create()
                    .name("idx_api_keys_user_id")
                    .table(api_key::Entity)
                    .col(api_key::Column::UserId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(url_click::Entity).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(api_key::Entity).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(shortened_url::Entity).to_owned())
            .await?;
        Ok(())
    }
}
