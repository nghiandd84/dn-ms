pub use sea_orm_migration::prelude::{async_trait, MigrationTrait, MigratorTrait};

pub mod m20260212_000001_create_booking_tables;
pub mod m20260213_000001_add_currency_to_bookings;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20260212_000001_create_booking_tables::Migration),
            Box::new(m20260213_000001_add_currency_to_bookings::Migration),
        ]
    }
}
