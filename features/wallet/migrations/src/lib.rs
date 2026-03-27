pub use sea_orm_migration::prelude::{async_trait, MigrationTrait, MigratorTrait};

pub mod m20260101_000001_create_wallet_tables;
pub mod m20260102_000001_create_top_up_transaction_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20260101_000001_create_wallet_tables::Migration),
            Box::new(m20260102_000001_create_top_up_transaction_table::Migration),
        ]
    }
}
