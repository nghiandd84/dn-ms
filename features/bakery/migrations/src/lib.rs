pub use sea_orm_migration::prelude::{async_trait, MigrationTrait, MigratorTrait};

mod m20250428_000001_create_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20250428_000001_create_table::Migration)]
    }
}
