use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{entity::prelude::*, ActiveValue};
use serde::{Deserialize, Serialize};
use shared_shared_macro::Dto;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize, Default, Dto)]
#[sea_orm(table_name = "template_translations")]
#[dto(
    name(TemplateTranslationForCreate),
    columns(template_id, language_code, subject, body, version_name)
)]
#[dto(
    name(TemplateTranslationForUpdate),
    columns(template_id, language_code, subject, body, version_name),
    option
)]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,

    #[sea_orm(column_name = "template_id")]
    pub template_id: i32,

    #[sea_orm(column_type = "String(StringLen::N(10))")]
    pub language_code: String,

    #[sea_orm(column_type = "String(StringLen::N(255))")]
    pub subject: String,

    #[sea_orm(column_type = "Text")]
    pub body: String,

    #[sea_orm(column_type = "String(StringLen::N(50))")]
    pub version_name: String,

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
    #[sea_orm(
        belongs_to = "super::email_templates::Entity",
        from = "Column::TemplateId",
        to = "super::email_templates::Column::Id"
    )]
    EmailTemplates,
}

impl Related<super::email_templates::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::EmailTemplates.def()
    }
}
