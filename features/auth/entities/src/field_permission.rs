use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{entity::prelude::*, ActiveValue, ConnectionTrait};
use serde::Serialize;

use shared_shared_macro::Dto;

#[derive(Debug, Clone, DeriveEntityModel, Serialize, Default, Dto)]
#[sea_orm(table_name = "field_permissions")]
#[dto(name(FieldPermissionForCreate), columns(role_id, resource, action, fields))]
#[dto(name(FieldPermissionForUpdate), columns(fields), option)]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub role_id: Uuid,
    #[sea_orm(column_type = "String(StringLen::N(255))")]
    pub resource: String,
    pub action: i32,
    #[sea_orm(column_type = "Text", array)]
    pub fields: Vec<String>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::role::Entity",
        from = "Column::RoleId",
        to = "super::role::Column::Id"
    )]
    Role,
}

impl Related<super::role::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Role.def()
    }
}

#[async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(mut self, _db: &C, insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        let current_time = Utc::now().naive_utc();
        self.updated_at = ActiveValue::Set(current_time);
        if insert {
            self.created_at = ActiveValue::Set(current_time);
        }
        Ok(self)
    }
}
