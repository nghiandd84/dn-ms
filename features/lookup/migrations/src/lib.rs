pub use sea_orm_migration::prelude::{async_trait, MigrationTrait, MigratorTrait};

mod m20260403_000001_create_lookup_tables;
mod m20260406_000002_add_meta_to_lookup_item;
mod m20260406_000003_add_default_to_lookup_item;
mod m20260407_000004_seed_countries;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20260403_000001_create_lookup_tables::Migration),
            Box::new(m20260406_000002_add_meta_to_lookup_item::Migration),
            Box::new(m20260406_000003_add_default_to_lookup_item::Migration),
            Box::new(m20260407_000004_seed_countries::Migration),
        ]
    }
}
