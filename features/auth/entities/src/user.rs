use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{entity::prelude::*, ActiveValue, ConnectionTrait};
use serde::{Deserialize, Serialize};
use shared_shared_macro::Dto;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, Default, Dto)]
#[sea_orm(table_name = "users")]
#[dto(name(UserForCreate), columns(email, first_name, last_name, password))]
#[dto(name(UserForUpdateProfile), columns(first_name, last_name), option)]
#[dto(
    name(UserForUpdateProfileOption),
    columns(id, first_name, last_name),
    option
)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    #[sea_orm(column_type = "String(StringLen::N(250))", unique)]
    pub email: String,
    #[sea_orm(column_type = "String(StringLen::N(50))")]
    pub first_name: String,
    #[sea_orm(column_type = "String(StringLen::N(50))")]
    pub last_name: String,
    #[sea_orm(default_value = false)]
    pub confirmed: bool,
    #[sea_orm(default_value = false)]
    pub two_factor_enabled: bool,
    #[sea_orm(default_value = 1)]
    pub version: i16,
    #[sea_orm(column_type = "Text")]
    pub password: String,
    pub is_active: bool,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::access::Entity")]
    Access,
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
            self.is_active = ActiveValue::Set(true);
            self.created_at = ActiveValue::Set(current_time);
        }
        Ok(self)
    }
}

impl Related<super::role::Entity> for Entity {
    fn to() -> RelationDef {
        super::access::Relation::Role.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::access::Relation::User.def().rev())
    }
}

impl Related<super::access::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Access.def()
    }
}
