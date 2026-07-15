use async_trait::async_trait;
use chrono::{NaiveDateTime as DateTime, Utc};
use sea_orm::{entity::prelude::*, ActiveValue, ConnectionTrait};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use shared_shared_macro::Dto;

use crate::shortened_url;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, Default, Dto)]
#[sea_orm(table_name = "url_clicks")]
#[dto(
    name(UrlClickForCreate),
    columns(url_id, ip_address, user_agent, referrer, country)
)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub url_id: Uuid,
    #[sea_orm(column_type = "String(StringLen::N(45))", nullable)]
    pub ip_address: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub user_agent: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub referrer: String,
    #[sea_orm(column_type = "String(StringLen::N(2))", nullable)]
    pub country: String,
    pub clicked_at: DateTime,
    pub created_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "shortened_url::Entity",
        from = "Column::UrlId",
        to = "shortened_url::Column::Id"
    )]
    ShortenedUrl,
}

impl Related<shortened_url::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ShortenedUrl.def()
    }
}

#[async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(mut self, _db: &C, insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        if insert {
            let now = Utc::now().naive_utc();
            self.id = ActiveValue::Set(Uuid::new_v4());
            self.clicked_at = ActiveValue::Set(now);
            self.created_at = ActiveValue::Set(now);
        }
        Ok(self)
    }
}
