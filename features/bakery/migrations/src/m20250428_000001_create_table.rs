// use features_auth_entities::user;
use sea_orm_migration::prelude::*;

use features_bakery_entities::{baker, bakery, cake, cakes_bakers, customer, lineitem, order};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20250428_000001_create_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let _bakery = manager
            .create_table(
                Table::create()
                    .table(bakery::Entity)
                    .col(
                        ColumnDef::new(bakery::Column::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(bakery::Column::Name).string().not_null())
                    .col(
                        ColumnDef::new(bakery::Column::ProfitMargin)
                            .double()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(baker::Column::CreatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(baker::Column::UpdatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        let _baker = manager
            .create_table(
                Table::create()
                    .table(baker::Entity)
                    .col(
                        ColumnDef::new(baker::Column::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(baker::Column::Name).string().not_null())
                    .col(
                        ColumnDef::new(baker::Column::ContactDetails)
                            .json()
                            .not_null(),
                    )
                    .col(ColumnDef::new(baker::Column::BakeryId).integer())
                    .col(
                        ColumnDef::new(baker::Column::CreatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(baker::Column::UpdatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-baker-bakery_id")
                            .from(baker::Entity, baker::Column::BakeryId)
                            .to(bakery::Entity, bakery::Column::Id)
                            .on_delete(ForeignKeyAction::SetNull)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        let _customer = manager
            .create_table(
                Table::create()
                    .table(customer::Entity)
                    .col(
                        ColumnDef::new(customer::Column::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(customer::Column::Name).string().not_null())
                    .col(ColumnDef::new(customer::Column::Notes).text())
                    .col(
                        ColumnDef::new(customer::Column::CreatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(customer::Column::UpdatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        let _order = manager
            .create_table(
                Table::create()
                    .table(order::Entity)
                    .col(
                        ColumnDef::new(order::Column::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(order::Column::Total)
                            .double()
                            .not_null(),
                    )
                    .col(ColumnDef::new(order::Column::BakeryId).integer().not_null())
                    .col(
                        ColumnDef::new(order::Column::CustomerId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(order::Column::PlacedAt)
                            .date_time()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-order-bakery_id")
                            .from(order::Entity, order::Column::BakeryId)
                            .to(bakery::Entity, bakery::Column::Id),
                    )
                    .col(
                        ColumnDef::new(order::Column::CreatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(order::Column::UpdatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-order-customer_id")
                            .from(order::Entity, order::Column::CustomerId)
                            .to(customer::Entity, customer::Column::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        let _cake = manager
            .create_table(
                Table::create()
                    .table(cake::Entity)
                    .col(
                        ColumnDef::new(cake::Column::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(cake::Column::Name).string().not_null())
                    .col(
                        ColumnDef::new(cake::Column::Price)
                            .double()
                            .not_null(),
                    )
                    .col(ColumnDef::new(cake::Column::BakeryId).integer())
                    .col(
                        ColumnDef::new(cake::Column::CreatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(cake::Column::UpdatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-cake-bakery_id")
                            .from(cake::Entity, cake::Column::BakeryId)
                            .to(bakery::Entity, bakery::Column::Id)
                            .on_delete(ForeignKeyAction::SetNull)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(
                        ColumnDef::new(cake::Column::GlutenFree)
                            .boolean()
                            .not_null(),
                    )
                    .col(ColumnDef::new(cake::Column::Serial).uuid().not_null())
                    .to_owned(),
            )
            .await?;

        let _cakes_bakers = manager
            .create_table(
                Table::create()
                    .table(cakes_bakers::Entity)
                    .col(
                        ColumnDef::new(cakes_bakers::Column::CakeId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(cakes_bakers::Column::BakerId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(cakes_bakers::Column::CreatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(cakes_bakers::Column::UpdatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .primary_key(
                        Index::create()
                            .name("pk-cakes_bakers")
                            .col(cakes_bakers::Column::CakeId)
                            .col(cakes_bakers::Column::BakerId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-cakes_bakers-cake_id")
                            .from(cakes_bakers::Entity, cakes_bakers::Column::CakeId)
                            .to(cake::Entity, cake::Column::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-cakes_bakers-baker_id")
                            .from(cakes_bakers::Entity, cakes_bakers::Column::BakerId)
                            .to(baker::Entity, baker::Column::Id),
                    )
                    .to_owned(),
            )
            .await?;

        let _lineitem = manager
            .create_table(
                Table::create()
                    .table(lineitem::Entity)
                    .col(
                        ColumnDef::new(lineitem::Column::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(lineitem::Column::Price)
                            .double()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(lineitem::Column::Quantity)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(lineitem::Column::OrderId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(lineitem::Column::CakeId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(lineitem::Column::CreatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(lineitem::Column::UpdatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-lineitem-order_id")
                            .from(lineitem::Entity, lineitem::Column::OrderId)
                            .to(order::Entity, order::Column::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-lineitem-cake_id")
                            .from(lineitem::Entity, lineitem::Column::CakeId)
                            .to(cake::Entity, cake::Column::Id),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let _lineitem = manager
            .drop_table(Table::drop().table(lineitem::Entity).to_owned())
            .await?;
        let _lineitem = manager
            .drop_table(Table::drop().table(lineitem::Entity).to_owned())
            .await?;
        let _cakes_bakers = manager
            .drop_table(Table::drop().table(cakes_bakers::Entity).to_owned())
            .await?;
        let _cake = manager
            .drop_table(Table::drop().table(cake::Entity).to_owned())
            .await?;
        let _order = manager
            .drop_table(Table::drop().table(order::Entity).to_owned())
            .await?;
        let _customer = manager
            .drop_table(Table::drop().table(customer::Entity).to_owned())
            .await?;
        let _baker = manager
            .drop_table(Table::drop().table(baker::Entity).to_owned())
            .await?;
        let _bakery = manager
            .drop_table(Table::drop().table(bakery::Entity).to_owned())
            .await?;

        Ok(())
    }
}
