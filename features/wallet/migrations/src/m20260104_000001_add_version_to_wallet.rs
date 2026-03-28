use sea_orm_migration::prelude::*;

use features_wallet_entities::wallet;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260104_000001_add_version_to_wallet"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(wallet::Entity)
                    .add_column(
                        ColumnDef::new(wallet::Column::Version)
                            .integer()
                            .not_null()
                            .default(1),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(wallet::Entity)
                    .drop_column(wallet::Column::Version)
                    .to_owned(),
            )
            .await
    }
}