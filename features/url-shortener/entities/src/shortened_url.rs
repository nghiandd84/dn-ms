use async_trait::async_trait;
use chrono::{NaiveDateTime as DateTime, Utc};
use sea_orm::{entity::prelude::*, ActiveValue, ConnectionTrait};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use shared_shared_macro::Dto;

use crate::url_click;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, Default, Dto)]
#[sea_orm(table_name = "shortened_urls")]
#[dto(
    name(ShortenedUrlForCreate),
    columns(user_id, original_url, short_code, title, expires_at)
)]
#[dto(name(ShortenedUrlForUpdate), columns(title, is_active, expires_at), option)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub user_id: Uuid,
    #[sea_orm(column_type = "Text")]
    pub original_url: String,
    #[sea_orm(column_type = "String(StringLen::N(30))", unique)]
    pub short_code: String,
    #[sea_orm(column_type = "String(StringLen::N(255))", nullable)]
    pub title: String,
    pub is_active: bool,
    pub expires_at: Option<DateTime>,
    pub click_count: i64,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "url_click::Entity")]
    UrlClick,
}

impl Related<url_click::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UrlClick.def()
    }
}

#[async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(mut self, _db: &C, insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        let current_time = Utc::now().naive_utc();
        if insert {
            self.id = ActiveValue::Set(Uuid::new_v4());
            self.created_at = ActiveValue::Set(current_time);
            if self.is_active.is_not_set() {
                self.is_active = ActiveValue::Set(true);
            }
            if self.click_count.is_not_set() {
                self.click_count = ActiveValue::Set(0);
            }
        }
        self.updated_at = ActiveValue::Set(current_time);
        Ok(self)
    }
}
