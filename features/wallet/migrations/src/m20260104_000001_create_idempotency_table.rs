use sea_orm_migration::prelude::*;

use features_wallet_entities::idempotency;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260104_000001_create_idempotency_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(idempotency::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(idempotency::Column::Id)
                            .uuid()
                            .primary_key()
                            .extra("DEFAULT public.uuid_generate_v4()"),
                    )
                    .col(ColumnDef::new(idempotency::Column::Key).string().not_null())
                    .col(
                        ColumnDef::new(idempotency::Column::Endpoint)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(idempotency::Column::RequestHash)
                            .text()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(idempotency::Column::ResponseStatus)
                            .integer()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(idempotency::Column::State)
                            .string()
                            .not_null()
                            .default("PENDING"),
                    )
                    .col(
                        ColumnDef::new(idempotency::Column::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(idempotency::Column::ExpiresAt)
                            .date_time()
                            .null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_idempotency_key")
                    .table(idempotency::Entity)
                    .col(idempotency::Column::Key)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_idempotency_expires")
                    .table(idempotency::Entity)
                    .col(idempotency::Column::ExpiresAt)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(idempotency::Entity).to_owned())
            .await?;
        Ok(())
    }
}
