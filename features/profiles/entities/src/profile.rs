use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{entity::prelude::*, ActiveValue, ConnectionTrait};
use serde::{Deserialize, Serialize};
use shared_shared_macro::Dto;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, Default, Dto)]
#[sea_orm(table_name = "profiles")]
#[dto(
    name(ProfileForCreate),
    columns(user_id, first_name, last_name, bio, avatar_url, location)
)]
#[dto(
    name(ProfileForUpdate),
    columns(first_name, last_name, bio, avatar_url, location),
    option
)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub user_id: Uuid,
    #[sea_orm(column_type = "String(StringLen::N(100))")]
    pub first_name: String,
    #[sea_orm(column_type = "String(StringLen::N(100))")]
    pub last_name: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub bio: String,
    #[sea_orm(column_type = "String(StringLen::N(500))", nullable)]
    pub avatar_url: String,
    #[sea_orm(column_type = "String(StringLen::N(255))", nullable)]
    pub location: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::user_preference::Entity")]
    UserPreference,
    #[sea_orm(has_many = "super::social_link::Entity")]
    SocialLink,
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
