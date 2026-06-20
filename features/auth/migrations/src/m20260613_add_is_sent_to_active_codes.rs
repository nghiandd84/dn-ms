use sea_orm_migration::prelude::*;

use features_auth_entities::active_code;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260613_add_is_sent_to_active_codes"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(active_code::Entity)
                    .add_column(
                        ColumnDef::new(active_code::Column::IsSent)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(active_code::Entity)
                    .drop_column(active_code::Column::IsSent)
                    .to_owned(),
            )
            .await
    }
}
