use features_auth_entities::active_code;
use features_auth_entities::client;
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20251227_add_user_id_to_active_code_table_and_client_key_to_client_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let _add_update_time = manager
            .alter_table(
                Table::alter()
                    .table(active_code::Entity)
                    .add_column_if_not_exists(
                        ColumnDef::new(active_code::Column::UserId)
                            .uuid()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await;

        let _add_update_time = manager
            .alter_table(
                Table::alter()
                    .table(client::Entity)
                    .add_column_if_not_exists(
                        ColumnDef::new(client::Column::ClientKey)
                            .string()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let _drop_user_id_column = manager
            .alter_table(
                Table::alter()
                    .table(active_code::Entity)
                    .drop_column(active_code::Column::UserId)
                    .to_owned(),
            )
            .await;

        let _drop_client_key_column = manager
            .alter_table(
                Table::alter()
                    .table(client::Entity)
                    .drop_column(client::Column::ClientKey)
                    .to_owned(),
            )
            .await;

        Ok(())
    }
}
