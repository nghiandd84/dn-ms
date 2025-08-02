use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "template_placeholders")]
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
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::email_templates::Entity",
        from = "Column::TemplateId",
        to = "super::email_templates::Column::Id"
    )]
    EmailTemplates,
}
