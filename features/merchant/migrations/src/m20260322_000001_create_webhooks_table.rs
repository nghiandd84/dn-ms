use sea_orm_migration::prelude::*;

use features_merchant_entities::webhook;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(webhook::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(webhook::Column::Id)
                            .uuid()
                            .extra("DEFAULT public.uuid_generate_v4()")
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(webhook::Column::MerchantId)
                            .string()
                            .string_len(50)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(webhook::Column::Url)
                            .string()
                            .string_len(255)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(webhook::Column::EventTypes)
                            .array(ColumnType::String(StringLen::N(128)))
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(webhook::Column::Secret)
                            .string()
                            .string_len(255)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(webhook::Column::Status)
                            .string()
                            .string_len(20)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(webhook::Column::CreatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(webhook::Column::UpdatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(webhook::Entity).to_owned())
            .await
    }
}
