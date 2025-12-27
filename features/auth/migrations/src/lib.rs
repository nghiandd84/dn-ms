pub use sea_orm_migration::prelude::{async_trait, MigrationTrait, MigratorTrait};

mod m20220101_000001_create_table;
mod m20220101_000002_create_id_version_index;
mod m20250424_create_role;
mod m20250425_create_access;
mod m20250426_create_access_key;
mod m20250725_upgrade_to_oauth2;
mod m20251031_create_authentication_requests;
mod m20251127_add_client_id_into_role;
mod m20251127_add_created_at_updated_at_to_access;
mod m20251225_add_active_code_table;
mod m20251227_add_user_id_to_active_code_table_and_client_key_to_client_table;

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
            Box::new(m20251031_create_authentication_requests::Migration),
            Box::new(m20251127_add_client_id_into_role::Migration),
            Box::new(m20251127_add_created_at_updated_at_to_access::Migration),
            Box::new(m20251225_add_active_code_table::Migration),
            Box::new(m20251227_add_user_id_to_active_code_table_and_client_key_to_client_table::Migration),
        ]
    }
}
