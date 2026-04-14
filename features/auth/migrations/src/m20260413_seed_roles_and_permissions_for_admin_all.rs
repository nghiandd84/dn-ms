use features_auth_entities::{permission, role, role_permission};
use sea_orm_migration::{
    prelude::{
        prelude::{Utc, Uuid},
        *,
    },
    sea_orm::{ActiveModelTrait, ActiveValue::Set},
};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260413_seed_roles_and_permissions_for_admin_all"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let now = Utc::now().naive_utc();
        let role_id = Uuid::new_v4();
        let perm_id = Uuid::new_v4();

        // Insert ADMIN_ALL role using ActiveModel
        let role = role::ActiveModel {
            id: Set(role_id),
            name: Set("ADMIN_ALL".to_string()),
            description: Set("Admin with all permissions".to_string()),
            created_at: Set(now),
            updated_at: Set(now),
            client_id: Set(Uuid::nil()),
            is_default: Set(false),
            ..Default::default()
        };
        role.insert(manager.get_connection()).await?;

        // Insert AUTH_PERMISSION permission using ActiveModel
        let permission = permission::ActiveModel {
            resource: Set("AUTH:PERMISSION".to_string()),
            description: Set(Some("Permission for all auth actions".to_string())),
            mask: Set(0),
            created_at: Set(now),
            updated_at: Set(now),
            ..Default::default()
        };
        permission.insert(manager.get_connection()).await?;

        // Link role and permission using ActiveModel
        let role_perm = role_permission::ActiveModel {
            id: Set(Uuid::new_v4()),
            role_id: Set(role_id),
            permission_id: Set(perm_id),
            created_at: Set(now),
            updated_at: Set(now),
            ..Default::default()
        };
        role_perm.insert(manager.get_connection()).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Remove the seeded data
        use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

        // Find the role and permission IDs
        let db = manager.get_connection();
        if let Some(role) = role::Entity::find()
            .filter(role::Column::Name.eq("ADMIN_ALL"))
            .one(db)
            .await?
        {
            // Delete role_permissions
            role_permission::Entity::delete_many()
                .filter(role_permission::Column::RoleId.eq(role.id))
                .exec(db)
                .await?;
            // Delete role
            role::Entity::delete_by_id(role.id).exec(db).await?;
        }
        if let Some(perm) = permission::Entity::find()
            .filter(permission::Column::Resource.eq("AUTH:PERMISSION"))
            .one(db)
            .await?
        {
            // Delete permission
            permission::Entity::delete_by_id(perm.id).exec(db).await?;
        }
        Ok(())
    }
}
