pub use sea_orm_migration::prelude::{async_trait, MigrationTrait, MigratorTrait};

pub mod m20260318_000001_create_merchant_tables;
pub mod m20260320_000001_create_api_keys_table;
pub mod m20260322_000001_create_webhooks_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20260318_000001_create_merchant_tables::Migration),
            Box::new(m20260320_000001_create_api_keys_table::Migration),
            Box::new(m20260322_000001_create_webhooks_table::Migration),
        ]
    }
}
