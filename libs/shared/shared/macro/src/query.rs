use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, punctuated::Punctuated, DeriveInput, Meta, Token};

pub fn query_impl(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    
    let mut key_type_str: String = String::new();

    let mut query_datas: Vec<String> = Vec::new();
    for attr in &input.attrs {
        if attr.path().is_ident("query") {
            let nested = attr
                .parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)
                .unwrap();
            for meta in nested {
                match meta {
                    Meta::List(meta_list) => {
                        let name = meta_list.path.get_ident().unwrap();
                        let tokens = meta_list.tokens.clone();
                        if name == "key_type" {
                            key_type_str = tokens.to_string();
                            break;
                        }
                    }

                    _ => {}
                }
            }
        }
        if attr.path().is_ident("query_filter") {
            let mut column_name: String = String::new();
            let nested = attr
                .parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)
                .unwrap();
            for meta in nested {
                match meta {
                    Meta::List(meta_list) => {
                        let name = meta_list.path.get_ident().unwrap();
                        let tokens = meta_list.tokens.clone();
                        if name == "column_name" {
                            let name = tokens.to_string();
                            column_name = name.to_owned();
                        }
                    }

                    _ => {}
                }
            }
            query_datas.push(column_name);
        }
    }

    let function_quotes = query_datas.iter().map(|column_name| {
        let fn_name = format_ident!("filter_condition_{}", column_name.to_lowercase());
        let column_name = format_ident!("{}", column_name);
        quote! { 
            fn #fn_name (column: #column_name, filter_enum: &FilterEnum) -> Condition {
                match filter_enum {
                    FilterEnum::Bool(filter) => match filter.operator {
                        FilterOperator::Equal => Condition::any().add(column.eq(filter.value.clone())),
                        FilterOperator::NotEqual => Condition::any().add(column.ne(filter.value.clone())),
                        _ => Condition::all(),
                    },

                    FilterEnum::U32(filter) => {
                        let values: Vec<u32> = filter
                            .raw_value
                            .split(",")
                            .filter_map(|s| u32::from_str(s).ok())
                            .collect();
                        let condition = match filter.operator {
                            FilterOperator::Less => Condition::any().add(column.lt(filter.value.clone())),
                            FilterOperator::LessEqual => Condition::any().add(column.lte(filter.value.clone())),
                            FilterOperator::Greater => Condition::any().add(column.gt(filter.value.clone())),
                            FilterOperator::GreaterEqual => Condition::any().add(column.gte(filter.value.clone())),
                            FilterOperator::Equal => Condition::any().add(column.eq(filter.value.clone())),
                            FilterOperator::NotEqual => Condition::any().add(column.ne(filter.value.clone())),
                            FilterOperator::In => Condition::any().add(column.is_in(values)),
                            FilterOperator::NotIn => Condition::any().add(column.is_not_in(values)),
                            _ => Condition::all(),
                        };
                        condition
                    }
                    FilterEnum::U64(filter) => {
                        let values: Vec<u64> = filter
                            .raw_value
                            .split(",")
                            .filter_map(|s| u64::from_str(s).ok())
                            .collect();
                        let condition = match filter.operator {
                            FilterOperator::Less => Condition::any().add(column.lt(filter.value.clone())),
                            FilterOperator::LessEqual => Condition::any().add(column.lte(filter.value.clone())),
                            FilterOperator::Greater => Condition::any().add(column.gt(filter.value.clone())),
                            FilterOperator::GreaterEqual => Condition::any().add(column.gte(filter.value.clone())),
                            FilterOperator::Equal => Condition::any().add(column.eq(filter.value.clone())),
                            FilterOperator::NotEqual => Condition::any().add(column.ne(filter.value.clone())),
                            FilterOperator::In => Condition::any().add(column.is_in(values)),
                            FilterOperator::NotIn => Condition::any().add(column.is_not_in(values)),
                            _ => Condition::all(),
                        };
                        condition
                    }
                    FilterEnum::F32(filter) => {
                        let values: Vec<f32> = filter
                            .raw_value
                            .split(",")
                            .filter_map(|s| f32::from_str(s).ok())
                            .collect();
                        let condition = match filter.operator {
                            FilterOperator::Less => Condition::any().add(column.lt(filter.value.clone())),
                            FilterOperator::LessEqual => Condition::any().add(column.lte(filter.value.clone())),
                            FilterOperator::Greater => Condition::any().add(column.gt(filter.value.clone())),
                            FilterOperator::GreaterEqual => Condition::any().add(column.gte(filter.value.clone())),
                            FilterOperator::Equal => Condition::any().add(column.eq(filter.value.clone())),
                            FilterOperator::NotEqual => Condition::any().add(column.ne(filter.value.clone())),
                            FilterOperator::In => Condition::any().add(column.is_in(values)),
                            FilterOperator::NotIn => Condition::any().add(column.is_not_in(values)),
                            _ => Condition::all(),
                        };
                        condition
                    }
                    FilterEnum::F64(filter) => {
                        let values: Vec<f64> = filter
                            .raw_value
                            .split(",")
                            .filter_map(|s| f64::from_str(s).ok())
                            .collect();
                        let condition = match filter.operator {
                            FilterOperator::Less => Condition::any().add(column.lt(filter.value.clone())),
                            FilterOperator::LessEqual => Condition::any().add(column.lte(filter.value.clone())),
                            FilterOperator::Greater => Condition::any().add(column.gt(filter.value.clone())),
                            FilterOperator::GreaterEqual => Condition::any().add(column.gte(filter.value.clone())),
                            FilterOperator::Equal => Condition::any().add(column.eq(filter.value.clone())),
                            FilterOperator::NotEqual => Condition::any().add(column.ne(filter.value.clone())),
                            FilterOperator::In => Condition::any().add(column.is_in(values)),
                            FilterOperator::NotIn => Condition::any().add(column.is_not_in(values)),
                            _ => Condition::all(),
                        };
                        condition
                    }
                    FilterEnum::Uuid(filter) => {
                        let values: Vec<Uuid> = filter
                            .raw_value
                            .split(",")
                            .filter_map(|s| Uuid::parse_str(s).ok())
                            .collect();
                        let condition = match filter.operator {
                            FilterOperator::Equal => Condition::any().add(column.eq(filter.value.clone())),
                            FilterOperator::NotEqual => Condition::any().add(column.ne(filter.value.clone())),
                            FilterOperator::In => Condition::any().add(column.is_in(values)),
                            FilterOperator::NotIn => Condition::any().add(column.is_not_in(values)),
                            _ => Condition::all(),
                        };
                        condition
                    }
                    FilterEnum::String(filter) => match filter.operator {
                        FilterOperator::Equal => Condition::any().add(column.eq(filter.value.clone())),
                        FilterOperator::NotEqual => Condition::any().add(column.ne(filter.value.clone())),
                        FilterOperator::Like => {
                            Condition::any().add(column.contains(filter.value.as_deref().unwrap_or_default()))
                        }
                        _ => Condition::all(),
                    },
                    FilterEnum::VecString(filter) => match filter.operator {
                        // ONLY for postgres, other database may not support array
                        FilterOperator::In => {
                            let values: Vec<String> = filter
                                    .raw_value
                                    .split(",")
                                    .map(|s| s.to_string())
                                    .collect();
                            let epxr = Expr::cust_with_exprs(
                                "$1  && $2 ::varchar[]",
                                vec![
                                    Expr::col(column).into(),
                                    Expr::value(values).into(),
                                ],
                            );
                            Condition::any().add(epxr)
                        },
                        FilterOperator::NotIn => {
                            let values: Vec<String> = filter
                                    .raw_value
                                    .split(",")
                                    .map(|s| s.to_string())
                                    .collect();
                            let epxr = Expr::cust_with_exprs(
                                "NOT ($1  @> $2 ::varchar[])",
                                vec![
                                    Expr::col(column).into(),
                                    Expr::value(values).into(),
                                ],
                            );
                            Condition::any().add(epxr)
                        }
                        _ => Condition::all(),
                    },
                    _ => Condition::all(),
                }
            }
        }
    });

    let get_by_id_quote = match key_type_str.as_str() {
        "Uuid" => quote! {
            async fn get_by_id_uuid(db: &DbConn, id: Uuid) -> Result<ModelOptionDto, DbErr> {
                let exists = Entity::find_by_id(id)
                    .one(db)
                    .await?
                    .ok_or(DbErr::RecordNotFound("Not found".to_string()))?;
                let model_option: ModelOptionDto = exists.into();
                Ok(model_option)
            }

            async fn get_by_id_i32(db: &DbConn, id: i32) -> Result<ModelOptionDto, DbErr> {
                unimplemented!("Not implemented")
            }
        },
        "i32" => quote! {
            async fn get_by_id_uuid(db: &DbConn, id: Uuid) -> Result<ModelOptionDto, DbErr> {
                unimplemented!("Not implemented")
            }

            async fn get_by_id_i32(db: &DbConn, id: i32) -> Result<ModelOptionDto, DbErr> {
                let exists = Entity::find_by_id(id)
                    .one(db)
                    .await?
                    .ok_or(DbErr::RecordNotFound("Not found".to_string()))?;
                let model_option: ModelOptionDto = exists.into();

                Ok(model_option)
            }
        },
        _ => quote! {},
    };

    let expanded = quote! {
        use std::str::FromStr;
        use sea_orm::{
            ConnectionTrait, DbConn, DbErr,
            entity::ColumnTrait, Condition, EntityTrait, Order as SeaOrder, PaginatorTrait, QueryFilter,
            QueryOrder, Select, QuerySelect, QueryTrait,
            sea_query::{Alias, SelectStatement, SimpleExpr, Expr},
            prelude::*
        };
        use shared_shared_data_core::{query::QueryManager, filter::FilterOperator, order::OrderDirection};

        impl #name {
            fn compute_pages_number(num_items: u64, page_size: u64) -> u64 {
                (num_items / page_size) + (num_items % page_size > 0) as u64
            }

            async fn get_num_items(db: &DbConn, query: &SelectStatement) -> Result<u64, DbErr> {
                let stmt = SelectStatement::new()
                    .expr(sea_orm::prelude::Expr::cust("COUNT(*) AS num_items"))
                    .from_subquery(
                        query
                            .clone()
                            .reset_limit()
                            .reset_offset()
                            .clear_order_by()
                            .to_owned(),
                        Alias::new("sub_query"),
                    )
                    .to_owned();
                let stmt = db.get_database_backend().build(&stmt);
                let num_items = match db.query_one(stmt).await? {
                    Some(res) => res.try_get::<i64>("", "num_items")? as u64,
                    None => 0,
                };
                Ok(num_items)
            }

            fn build_query(order: &Order, filters: &Vec<FilterEnum>) -> Select<Entity> {
                let default_order = Entity::find().order_by(Column::CreatedAt, SeaOrder::Desc);

                let select = match (order.order_name.as_deref(), order.order_direction.as_ref()) {
                    (Some(name), Some(direction)) => {
                        if let Ok(column) = Column::from_str(name) {
                            match direction {
                                OrderDirection::Asc => Entity::find().order_by(column, SeaOrder::Asc),
                                OrderDirection::Desc => Entity::find().order_by(column, SeaOrder::Desc),
                            }
                        } else {
                            default_order
                        }
                    }
                    _ => default_order,
                };
                /*
                let mut condition = Condition::all();
                for filter_enum in filters {
                    if let Ok(column) = Column::from_str(filter_enum.get_name().as_str()) {
                        condition = condition.add(Self::filter_condition_column(column, filter_enum));
                    }
                }

                let select = select.filter(condition);
                 */
                let condition = Self::build_filter_condition(filters);
                let select = select.filter(condition);

                select
            }
            
            #(#function_quotes)*  
        }

        impl QueryManager<ActiveModel, ModelOptionDto> for #name {
            
            #get_by_id_quote

            async fn filter(
                db: &DbConn,
                pagination: &Pagination,
                order: &Order,
                filter: &Vec<FilterEnum>,
            ) -> Result<QueryResult<ModelOptionDto>, DbErr> {
                let page_size = pagination.page_size.unwrap_or(1);
                let page = pagination.page.unwrap_or(1);
                let paginator = Self::build_query(order, filter).paginate(db, page_size);
                let num_pages = paginator.num_pages().await?;
                let result = paginator.fetch_page(page - 1).await?;
                let result: Vec<ModelOptionDto> = result.into_iter().map(|m| m.into()).collect();

                let page_result = QueryResult {
                    total_page: num_pages,
                    result: result,
                };
                Ok(page_result)
            }

            
        }

    };

    TokenStream::from(expanded)
}
