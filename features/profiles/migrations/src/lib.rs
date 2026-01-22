pub use sea_orm_migration::prelude::{async_trait, MigrationTrait, MigratorTrait};

mod m20260122_000001_create_profiles_table;
mod m20260122_000002_create_user_preferences_table;
mod m20260122_000003_create_social_links_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20260122_000001_create_profiles_table::Migration),
            Box::new(m20260122_000002_create_user_preferences_table::Migration),
            Box::new(m20260122_000003_create_social_links_table::Migration),
        ]
    }
}
