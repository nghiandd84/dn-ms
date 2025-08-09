use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{entity::prelude::*, ActiveValue};
use serde::{Deserialize, Serialize};
use shared_shared_macro::Dto;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize, Default, Dto)]
#[sea_orm(table_name = "template_placeholders")]
#[dto(
    name(TemplatePlaceholderForCreate),
    columns(template_id, placeholder_key, description, example_value, is_required)
)]
#[dto(
    name(TemplatePlaceholderForUpdate),
    columns(placeholder_key, description, example_value, is_required),
    option
)]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,

    #[sea_orm(column_name = "template_id")]
    pub template_id: i32,

    #[sea_orm(column_type = "String(StringLen::N(100))")]
    pub placeholder_key: String,

    #[sea_orm(column_type = "String(StringLen::N(255))", nullable)]
    pub description: String,

    #[sea_orm(column_type = "String(StringLen::N(255))", nullable)]
    pub example_value: String,

    #[sea_orm(default_value = false)]
    pub is_required: bool,

    pub created_at: DateTime,
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
