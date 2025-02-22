use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{entity::prelude::*, ActiveValue};
use serde::Serialize;

use shared_shared_macro::Dto;
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Default, Dto)]
#[sea_orm(table_name = "baker")]
#[dto(name(BakerForCreate), columns(name, contact_details, bakery_id))]
#[dto(
    name(BakerForUpdate),
    columns(name, contact_details, bakery_id),
    option
)]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub contact_details: Json,
    pub bakery_id: i32,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::bakery::Entity",
        from = "Column::BakeryId",
        to = "super::bakery::Column::Id",
        on_update = "Cascade",
        on_delete = "SetNull"
    )]
    Bakery,
}



impl Related<super::bakery::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Bakery.def()
    }
}

impl Related<super::cake::Entity> for Entity {
    fn to() -> RelationDef {
        super::cakes_bakers::Relation::Cake.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::cakes_bakers::Relation::Baker.def().rev())
    }
}

pub struct BakedForCustomer;

impl Linked for BakedForCustomer {
    type FromEntity = Entity;

    type ToEntity = super::customer::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![
            super::cakes_bakers::Relation::Baker.def().rev(),
            super::cakes_bakers::Relation::Cake.def(),
            super::lineitem::Relation::Cake.def().rev(),
            super::lineitem::Relation::Order.def(),
            super::order::Relation::Customer.def(),
        ]
    }
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
