use features_auth_entities::access;
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20250426_create_access_key"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(access::Entity)
                    .add_column(
                        ColumnDef::new(access::Column::Key)
                            .string_len(255)
                            .default("")
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(access::Entity)
                    .drop_column(access::Column::Key)
                    .to_owned(),
            )
            .await
    }
}
