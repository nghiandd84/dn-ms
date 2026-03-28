pub use sea_orm_migration::prelude::{async_trait, MigrationTrait, MigratorTrait};

pub mod m20260101_000001_create_wallet_tables;
pub mod m20260102_000001_create_top_up_transaction_table;
pub mod m20260103_000001_create_p2p_and_withdrawal_table;
pub mod m20260104_000001_create_idempotency_table;
pub mod m20260104_000001_add_version_to_wallet;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20260101_000001_create_wallet_tables::Migration),
            Box::new(m20260102_000001_create_top_up_transaction_table::Migration),
            Box::new(m20260103_000001_create_p2p_and_withdrawal_table::Migration),
            Box::new(m20260104_000001_create_idempotency_table::Migration),
            Box::new(m20260104_000001_add_version_to_wallet::Migration),
        ]
    }
}
