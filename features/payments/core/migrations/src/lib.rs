pub use sea_orm_migration::prelude::{async_trait, MigrationTrait, MigratorTrait};

pub mod m20260307_000001_create_payment_tables;
pub mod m20260310_000001_change_transaction_id_type_in_payment_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20260307_000001_create_payment_tables::Migration),
            Box::new(m20260310_000001_change_transaction_id_type_in_payment_table::Migration),
        ]
    }
}
