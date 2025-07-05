use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{entity::prelude::*, ActiveValue, ConnectionTrait};
use serde::Serialize;

use shared_shared_macro::Dto;

#[derive(Debug, Clone, DeriveEntityModel, Serialize, Default, Dto)]
#[sea_orm(table_name = "roles")]
#[dto(name(RoleForCreate), columns(name, description))]
#[dto(name(RoleForUpdate), columns(name, description), option)]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    #[sea_orm(column_type = "String(StringLen::N(250))", unique)]
    pub name: String,
    #[sea_orm(column_type = "String(StringLen::N(250))", unique)]
    pub description: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::access::Entity")]
    Access,
}

impl Related<super::access::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Access.def()
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

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        super::access::Relation::User.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::access::Relation::Role.def().rev())
    }
}
