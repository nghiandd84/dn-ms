use sea_orm::entity::prelude::*;
use serde::Serialize;

use shared_shared_macro::Dto;
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Default, Dto)]
#[sea_orm(table_name = "lineitem")]
#[dto(name(LineitemForCreate), columns(price, quantity, order_id, cake_id))]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub price: f64,
    pub quantity: i32,
    pub order_id: i32,
    pub cake_id: i32,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::order::Entity",
        from = "Column::OrderId",
        to = "super::order::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Order,
    #[sea_orm(
        belongs_to = "super::cake::Entity",
        from = "Column::CakeId",
        to = "super::cake::Column::Id"
    )]
    Cake,
}

impl Related<super::order::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Order.def()
    }
}

impl Related<super::cake::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Cake.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
