use features_bakery_model::baker::BakerData;
// use sea_orm::prelude::*;
use sea_orm::SelectTwoMany;
use uuid::Uuid;

use shared_shared_data_core::{
    filter::FilterEnum,
    order::Order,
    paging::{Pagination, QueryResult},
};
use shared_shared_macro::Query;

use features_bakery_entities::baker::{ActiveModel, Column, Entity, ModelOptionDto};
use features_bakery_entities::bakery::{
    Column as BakeryColumn, Entity as BakeryEntity, ModelOptionDto as BakeryModelOptionDto,
};

#[derive(Query)]
#[query(key_type(i32))]
#[query_filter(column_name(Column))]
#[query_filter(column_name(BakeryColumn))]
struct BakerQueryManager;

impl BakerQueryManager {
    fn build_filter_condition(filters: &Vec<FilterEnum>) -> Condition {
        let mut condition = Condition::all();
        for filter_enum in filters {
            let name = filter_enum.get_name();
            let column_name = name.as_str();
            if column_name.starts_with("bakery.") {
                let bakery_column_name = &column_name[7..];
                if let Ok(column) = BakeryColumn::from_str(bakery_column_name) {
                    condition =
                        condition.add(Self::filter_condition_bakerycolumn(column, filter_enum));
                }
            } else {
                if let Ok(column) = Column::from_str(column_name) {
                    condition = condition.add(Self::filter_condition_column(column, filter_enum));
                }
            }
        }
        condition
    }

    async fn advance_search(
        db: &DbConn,
        pagination: &Pagination,
        order: &Order,
        filter: &Vec<FilterEnum>,
    ) -> Result<QueryResult<(ModelOptionDto, BakeryModelOptionDto)>, DbErr> {
        let page_size = pagination.page_size.unwrap_or(1);
        let page = pagination.page.unwrap_or(1);
        tracing::debug!("page {} page_size {}", page, page_size);
        let select = Self::advance_select(order, filter)
            .limit(page_size)
            .offset((page - 1) * page_size);
        let num_items = Self::get_num_items(db, select.as_query()).await?;
        let result: Vec<(ModelOptionDto, BakeryModelOptionDto)> = select
            .all(db)
            .await?
            .into_iter()
            .map(|(baker, bakeries)| {
                let bakery = bakeries.into_iter().next();
                (baker.into(), bakery.unwrap().into())
            })
            .collect();

        let page_result = QueryResult {
            total_page: Self::compute_pages_number(num_items, page_size),
            result: result,
        };

        // let s1 = Entity::find().find_also_related(BakeryEntity);
        // let x = Self::get_num_items(db, s1.as_query()).await?;
        // tracing::debug!(x);

        let s1 = Entity::find_by_id(1).one(db).await?.unwrap();
        let e1 = s1.find_related(BakeryEntity).all(db).await?;
        tracing::debug!("e1: {:?}", e1);
        let e2 = s1.find_related(BakeryEntity).all(db).await?;
        tracing::debug!("e2: {:?}", e2);
        Ok(page_result)
    }

    fn advance_select(
        order: &Order,
        filters: &Vec<FilterEnum>,
    ) -> SelectTwoMany<Entity, BakeryEntity> {
        let mut select = Entity::find().find_with_related(BakeryEntity);
        // .left_join(BakeryEntity)
        // .select_also(BakeryEntity);
        // .find_also_related(BakeryEntity);
        // Same with .left_join(BakeryEntity).select_also(BakeryEntity);

        select = match (order.order_name.as_deref(), order.order_direction.as_ref()) {
            (Some(name), Some(direction)) => {
                if let Ok(column) = Column::from_str(name) {
                    match direction {
                        OrderDirection::Asc => select.order_by(column, SeaOrder::Asc),
                        OrderDirection::Desc => select.order_by(column, SeaOrder::Desc),
                    }
                } else {
                    select.order_by(Column::CreatedAt, SeaOrder::Desc)
                }
            }
            _ => select.order_by(Column::CreatedAt, SeaOrder::Desc),
        };
        let condition = Self::build_filter_condition(filters);
        select.filter(condition)
    }
}

pub struct BakerQuery {}

impl BakerQuery {
    pub async fn get_by_id<'a>(db: &'a DbConn, id: i32) -> Result<BakerData, DbErr> {
        let model = BakerQueryManager::get_by_id_i32(db, id).await?;
        let baker: BakerData = model.into();
        Ok(baker)
    }
    pub async fn search<'a>(
        db: &'a DbConn,
        pagination: &Pagination,
        order: &Order,
        filters: &Vec<FilterEnum>,
    ) -> Result<QueryResult<BakerData>, DbErr> {
        let result = BakerQueryManager::advance_search(db, pagination, order, filters).await?;
        let mapped_result = QueryResult {
            total_page: result.total_page,
            result: result.result.into_iter().map(|m| m.into()).collect(),
        };
        Ok(mapped_result)
    }
}
