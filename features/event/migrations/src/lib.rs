pub use sea_orm_migration::prelude::{async_trait, MigrationTrait, MigratorTrait};

pub use m20260211_000001_create_events_table::Migration;

mod m20260211_000001_create_events_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20260211_000001_create_events_table::Migration)]
    }
}
