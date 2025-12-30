pub use sea_orm_migration::prelude::{async_trait, MigrationTrait, MigratorTrait};

mod m20250801_create_email_template_tables;
mod m20251229_add_key_to_email_templates;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250801_create_email_template_tables::Migration),
            Box::new(m20251229_add_key_to_email_templates::Migration),
        ]
    }
}
