use features_auth_entities::role;
use sea_orm_migration::{prelude::*, sea_orm::prelude::Uuid};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20251127_add_client_id_into_role"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(role::Entity)
                    .add_column(
                        ColumnDef::new(role::Column::ClientId)
                            .uuid()
                            .not_null()
                            .default(Uuid::nil()),
                    )
                    .add_column(
                        ColumnDef::new(role::Column::IsDefault)
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
                    .table(role::Entity)
                    .drop_column(role::Column::ClientId)
                    .drop_column(role::Column::IsDefault)
                    .to_owned(),
            )
            .await
    }
}
