pub use sea_orm_migration::prelude::*;

mod m20260128_000001_create_projects_table;
mod m20260128_000002_create_translation_keys_table;
mod m20260128_000003_create_tags_table;
mod m20260128_000004_create_key_tags_table;
mod m20260128_000005_create_translation_versions_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20260128_000001_create_projects_table::Migration),
            Box::new(m20260128_000002_create_translation_keys_table::Migration),
            Box::new(m20260128_000003_create_tags_table::Migration),
            Box::new(m20260128_000004_create_key_tags_table::Migration),
            Box::new(m20260128_000005_create_translation_versions_table::Migration),
        ]
    }
}
