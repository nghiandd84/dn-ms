pub use sea_orm_migration::prelude::{async_trait, MigrationTrait, MigratorTrait};

mod m20220101_000001_create_table;
mod m20220101_000002_create_id_version_index;
mod m20250424_create_role;
mod m20250425_create_access;
mod m20250426_create_access_key;
mod m20250725_upgrade_to_oauth2;


pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20220101_000002_create_id_version_index::Migration),
            Box::new(m20250424_create_role::Migration),
            Box::new(m20250425_create_access::Migration),
            Box::new(m20250426_create_access_key::Migration),
            Box::new(m20250725_upgrade_to_oauth2::Migration),
        ]
    }
}
