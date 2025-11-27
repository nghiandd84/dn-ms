use features_auth_entities::access;
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20251127_add_created_at_updated_at_to_access_and_add_unique_constraint"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let _add_update_time = manager
            .alter_table(
                Table::alter()
                    .table(access::Entity)
                    .add_column_if_not_exists(
                        ColumnDef::new(access::Column::CreatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .add_column_if_not_exists(
                        ColumnDef::new(access::Column::UpdatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await;
        // add constrain to check duplicate role_id and user_id
        let index = Index::create()
            .name("idx-unique-user-role-pair") // A descriptive name for the constraint
            .table(access::Entity)
            .col(access::Column::UserId)
            .col(access::Column::RoleId)
            .unique() // This is the crucial part that enforces the constraint
            .to_owned();

        let _constraint = manager.create_index(index).await;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let _drop_constraint = manager
            .drop_index(
                Index::drop()
                    .name("idx-unique-user-role-pair")
                    .table(access::Entity)
                    .to_owned(),
            )
            .await;
        let _drop_update_time = manager
            .alter_table(
                Table::alter()
                    .table(access::Entity)
                    .drop_column(access::Column::CreatedAt)
                    .drop_column(access::Column::UpdatedAt)
                    .to_owned(),
            )
            .await;

        Ok(())
    }
}
