use sea_orm::entity::prelude::*;
use serde::Serialize;

use shared_shared_macro::Dto;
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Default, Dto)]
#[sea_orm(table_name = "customer")]
#[dto(
    name(CustomerForCreate),
    columns(name, notes)
)]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub notes: Option<String>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::order::Entity")]
    Order,
}

impl Related<super::order::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Order.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
