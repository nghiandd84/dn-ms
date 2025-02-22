use sea_orm::entity::prelude::*;
use serde::Serialize;

use shared_shared_macro::Dto;

#[derive(Clone, Debug, PartialEq, Default, DeriveEntityModel, Dto)]
#[sea_orm(table_name = "cakes_bakers")]
#[dto(
    name(CakeBakerForCreate),
    columns(cake_id, baker_id)
)]
pub struct Model {
    #[sea_orm(primary_key)]
    pub cake_id: i32,
    #[sea_orm(primary_key)]
    pub baker_id: i32,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::cake::Entity",
        from = "Column::CakeId",
        to = "super::cake::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Cake,
    #[sea_orm(
        belongs_to = "super::baker::Entity",
        from = "Column::BakerId",
        to = "super::baker::Column::Id"
    )]
    Baker,
}

impl ActiveModelBehavior for ActiveModel {}
