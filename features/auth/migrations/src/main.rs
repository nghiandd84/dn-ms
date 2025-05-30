use features_auth_migrations::Migrator;
use sea_orm_migration::prelude::*;

#[async_std::main]
async fn main() {
    cli::run_cli(Migrator).await;
}
