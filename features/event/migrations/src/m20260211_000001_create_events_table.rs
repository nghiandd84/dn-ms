use features_event_entities::event;
use sea_orm_migration::prelude::*;

// Import the Postgres-specific Type extension
use sea_orm_migration::prelude::sea_query::extension::postgres::Type;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260211_000001_create_events_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        /*
        // Create the event_status enum type
        // We can view values in psql with: "select * from pg_enum WHERE enumtypid = 'event_status'::regtype;"
        let _event_status_enum = manager
            .create_type(
                Type::create()
                    .as_enum(Alias::new("event_status"))
                    .values([
                        Alias::new("UPCOMING"),
                        Alias::new("ON_SALE"),
                        Alias::new("SOLD_OUT"),
                        Alias::new("CANCELLED"),
                    ])
                    .to_owned(),
            )
            .await;
        */
        let _ = manager
            .create_table(
                Table::create()
                    .table(event::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(event::Column::Id)
                            .uuid()
                            .extra("DEFAULT gen_random_uuid()")
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(event::Column::EventName)
                            .string()
                            .string_len(255)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(event::Column::EventDate)
                            .date_time()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(event::Column::VenueName)
                            .string()
                            .string_len(255)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(event::Column::TotalSeats)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(event::Column::Status)
                            // .custom(Alias::new("event_status"))
                            .string()
                            .string_len(100)
                            .not_null()
                            .default("UPCOMING"),
                    )
                    .col(
                        ColumnDef::new(event::Column::SaleStartTime)
                            .date_time()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(event::Column::CreatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(event::Column::UpdatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await;

        let _idx_sale_start_time = manager
            .create_index(
                Index::create()
                    .name("idx_sale_start_time")
                    .table(event::Entity)
                    .col(event::Column::SaleStartTime)
                    .to_owned(),
            )
            .await;

        let _idx_status = manager
            .create_index(
                Index::create()
                    .name("idx_status")
                    .table(event::Entity)
                    .col(event::Column::Status)
                    .to_owned(),
            )
            .await;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let _drop_idx_sale_start_time = manager
            .drop_index(
                Index::drop()
                    .name("idx_sale_start_time")
                    .table(event::Entity)
                    .to_owned(),
            )
            .await?;
        let _drop_idx_status = manager
            .drop_index(
                Index::drop()
                    .name("idx_status")
                    .table(event::Entity)
                    .to_owned(),
            )
            .await?;
        let _drop_table = manager
            .drop_table(Table::drop().table(event::Entity).to_owned())
            .await?;
        /*
        let _drop_event_status_enum = manager
            .drop_type(
                Type::drop()
                    .name(Alias::new("event_status"))
                    .if_exists()
                    .to_owned(),
            )
            .await?;
        */
        Ok(())
    }
}
