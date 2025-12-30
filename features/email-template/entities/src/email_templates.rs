use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{entity::prelude::*, ActiveValue};
use serde::{Deserialize, Serialize};
use shared_shared_macro::Dto;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize, Default, Dto)]
#[sea_orm(table_name = "email_templates")]
#[dto(
    name(EmailTemplateForCreate),
    columns(name, description, key, user_id, is_active)
)]
#[dto(
    name(EmailTemplateForUpdate),
    columns(name, description, key, is_active),
    option
)]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,

    #[sea_orm(column_type = "String(StringLen::N(255))", unique)]
    pub name: String,

    #[sea_orm(column_type = "Text", nullable)]
    pub description: String,

    #[sea_orm(column_type = "Text", unique)]
    pub key: String,

    #[sea_orm(default_value = true)]
    pub is_active: bool,

    #[sea_orm(column_type = "Uuid")]
    pub user_id: Uuid,

    pub created_at: DateTime,

    #[sea_orm(updated_at)]
    pub updated_at: DateTime,
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

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::template_translations::Entity")]
    TemplateTranslations,
}

impl Related<super::template_translations::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TemplateTranslations.def()
    }
}
