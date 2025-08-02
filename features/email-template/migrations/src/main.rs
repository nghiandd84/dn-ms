use sea_orm_migration::prelude::*;

use features_email_template_migrations::Migrator;

#[async_std::main]
async fn main() {
    cli::run_cli(Migrator).await;
}
