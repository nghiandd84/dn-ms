use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, punctuated::Punctuated, DeriveInput, Meta, Token};

struct RelatedEntityDef {
    entity_tokens: proc_macro2::TokenStream,
    field_name: String,
    include_name: String,
}

pub fn query_impl(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let mut key_type_str: String = String::new();
    let mut query_datas: Vec<String> = Vec::new();
    let mut related_entities: Vec<RelatedEntityDef> = Vec::new();

    for attr in &input.attrs {
        if attr.path().is_ident("query") {
            let nested = attr
                .parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)
                .unwrap();
            for meta in nested {
                if let Meta::List(meta_list) = meta {
                    if meta_list.path.get_ident().unwrap() == "key_type" {
                        key_type_str = meta_list.tokens.to_string();
                        break;
                    }
                }
            }
        }
        if attr.path().is_ident("query_filter") {
            let mut column_name: String = String::new();
            let nested = attr
                .parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)
                .unwrap();
            for meta in nested {
                if let Meta::List(meta_list) = meta {
                    let ident = meta_list.path.get_ident().unwrap();
                    if ident == "column_name" {
                        column_name = meta_list.tokens.to_string();
                    }
                }
            }
            query_datas.push(column_name);
        }
        if attr.path().is_ident("query_related") {
            let mut entity_tokens: Option<proc_macro2::TokenStream> = None;
            let mut field_name: Option<String> = None;
            let mut include_name: Option<String> = None;
            let nested = attr
                .parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)
                .unwrap();
            for meta in nested {
                if let Meta::List(meta_list) = meta {
                    let ident = meta_list.path.get_ident().unwrap();
                    let tokens = meta_list.tokens.clone();
                    if ident == "entity" {
                        entity_tokens = Some(tokens);
                    } else if ident == "field" {
                        field_name = Some(tokens.to_string());
                    } else if ident == "name" {
                        // Strip quotes from string literal
                        let s = tokens.to_string();
                        include_name = Some(s.trim_matches('"').to_string());
                    }
                }
            }
            if let (Some(et), Some(fn_)) = (entity_tokens, field_name) {
                let inc = include_name.unwrap_or_else(|| fn_.clone());
                related_entities.push(RelatedEntityDef {
                    entity_tokens: et,
                    field_name: fn_,
                    include_name: inc,
                });
            }
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
                    FilterEnum::I32(filter) => {
                        let values: Vec<i32> = filter
                            .raw_value
                            .split(",")
                            .filter_map(|s| i32::from_str(s).ok())
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
                        FilterOperator::StartWith => Condition::any().add(column.starts_with(filter.value.as_deref().unwrap_or_default())),
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
                                    Expr::col((Entity, column)).into(),
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
                                    Expr::col((Entity, column)).into(),
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
            #[tracing::instrument]
            async fn get_by_id_uuid(id: Uuid) -> Result<ModelOptionDto, DbErr> {
                let exists = Entity::find_by_id(id)
                    .one(Self::get_db())
                    .await?
                    .ok_or(DbErr::RecordNotFound("Not found".to_string()))?;
                let model_option: ModelOptionDto = exists.into();
                Ok(model_option)
            }

            async fn get_by_id_i32(id: i32) -> Result<ModelOptionDto, DbErr> {
                unimplemented!("Not implemented")
            }
            async fn get_by_id_str(id: String) -> Result<ModelOptionDto, DbErr> {
                unimplemented!("Not implemented")
            }
        },
        "i32" => quote! {
            #[tracing::instrument]
            async fn get_by_id_i32(id: i32) -> Result<ModelOptionDto, DbErr> {
                let exists = Entity::find_by_id(id)
                    .one(Self::get_db())
                    .await?
                    .ok_or(DbErr::RecordNotFound("Not found".to_string()))?;
                let model_option: ModelOptionDto = exists.into();

                Ok(model_option)
            }
            async fn get_by_id_uuid(id: Uuid) -> Result<ModelOptionDto, DbErr> {
                unimplemented!("Not implemented")
            }
            async fn get_by_id_str(id: String) -> Result<ModelOptionDto, DbErr> {
                unimplemented!("Not implemented")
            }
        },
        "String" => quote! {
            #[tracing::instrument]
            async fn get_by_id_str(id: String) -> Result<ModelOptionDto, DbErr> {
                let exists = Entity::find_by_id(id)
                    .one(Self::get_db())
                    .await?
                    .ok_or(DbErr::RecordNotFound("Not found".to_string()))?;
                let model_option: ModelOptionDto = exists.into();

                Ok(model_option)
            }

            async fn get_by_id_i32(id: i32) -> Result<ModelOptionDto, DbErr> {
                unimplemented!("Not implemented")
            }
            async fn get_by_id_uuid(id: Uuid) -> Result<ModelOptionDto, DbErr> {
                unimplemented!("Not implemented")
            }
        },
        _ => quote! {},
    };

    // Generate per-relation loading blocks for single entity by id
    let related_load_by_id_blocks: Vec<_> = related_entities.iter().map(|rel| {
        let entity_tok = &rel.entity_tokens;
        let field_ident = format_ident!("{}", rel.field_name);
        let inc_name = &rel.include_name;
        quote! {
            if includes.iter().any(|s| s == #inc_name) {
                let related = parent_model
                    .find_related(#entity_tok)
                    .all(Self::get_db())
                    .await?;
                model.#field_ident = Some(related.into_iter().map(|r| r.into()).collect());
            }
        }
    }).collect();

    // Generate per-relation loading blocks, each gated by includes check
    let related_load_blocks: Vec<_> = related_entities.iter().map(|rel| {
        let entity_tok = &rel.entity_tokens;
        let field_ident = format_ident!("{}", rel.field_name);
        let inc_name = &rel.include_name;
        quote! {
            if includes.iter().any(|s| s == #inc_name) {
                let with_related = Entity::find()
                    .filter(Column::Id.is_in(parent_ids.clone()))
                    .find_with_related(#entity_tok)
                    .all(Self::get_db())
                    .await?;
                for (parent, related) in with_related {
                    if let Some(dto) = result_map.get_mut(&parent.id) {
                        dto.#field_ident = Some(related.into_iter().map(|r| r.into()).collect());
                    }
                }
            }
        }
    }).collect();

    let related_entity_trait_quote = if !related_load_blocks.is_empty() {
        let get_by_id_with_related_quote = match key_type_str.as_str() {
            "Uuid" => quote! {
                async fn get_by_id_uuid_with_related_entities(
                    id: Uuid,
                    includes: &Vec<String>,
                ) -> Result<ModelOptionDto, DbErr> {
                    let parent_model = Entity::find_by_id(id)
                        .one(Self::get_db())
                        .await?
                        .ok_or(DbErr::RecordNotFound("Not found".to_string()))?;
                    let mut model: ModelOptionDto = parent_model.clone().into();
                    #(#related_load_by_id_blocks)*
                    Ok(model)
                }
            },
            "i32" => quote! {
                async fn get_by_id_i32_with_related_entities(
                    id: i32,
                    includes: &Vec<String>,
                ) -> Result<ModelOptionDto, DbErr> {
                    let parent_model = Entity::find_by_id(id)
                        .one(Self::get_db())
                        .await?
                        .ok_or(DbErr::RecordNotFound("Not found".to_string()))?;
                    let mut model: ModelOptionDto = parent_model.clone().into();
                    #(#related_load_by_id_blocks)*
                    Ok(model)
                }
            },
            "String" => quote! {
                async fn get_by_id_str_with_related_entities(
                    id: String,
                    includes: &Vec<String>,
                ) -> Result<ModelOptionDto, DbErr> {
                    let parent_model = Entity::find_by_id(id)
                        .one(Self::get_db())
                        .await?
                        .ok_or(DbErr::RecordNotFound("Not found".to_string()))?;
                    let mut model: ModelOptionDto = parent_model.clone().into();
                    #(#related_load_by_id_blocks)*
                    Ok(model)
                }
            },
            _ => quote! {},
        };

        quote! {
            async fn filter_with_related_entities(
                pagination: &Pagination,
                order: &Order,
                filter: &Vec<FilterEnum>,
                includes: &Vec<String>,
            ) -> Result<QueryResult<ModelOptionDto>, DbErr> {
                let page_size = pagination.page_size.unwrap_or(1);
                let mut page = pagination.page.unwrap_or(1);
                if page <= 0 {
                    page = 1;
                }
                let paginator = Self::build_query(order, filter).paginate(Self::get_db(), page_size);
                let num_pages = paginator.num_pages().await?;
                let parents = paginator.fetch_page(page - 1).await?;

                // Preserve order and build lookup map
                let parent_ids: Vec<_> = parents.iter().map(|p| p.id.clone()).collect();
                let mut result_map: std::collections::HashMap<_, ModelOptionDto> = parents
                    .into_iter()
                    .map(|p| {
                        let id = p.id.clone();
                        (id, p.into())
                    })
                    .collect();

                #(#related_load_blocks)*

                // Return in original order
                let result: Vec<ModelOptionDto> = parent_ids
                    .iter()
                    .filter_map(|id| result_map.remove(id))
                    .collect();

                Ok(QueryResult {
                    total_page: num_pages,
                    result,
                })
            }

            #get_by_id_with_related_quote
        }
    } else {
        quote! {}
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
        use shared_shared_config::db::DB_READ;


        impl #name {
            fn compute_pages_number(num_items: u64, page_size: u64) -> u64 {
                (num_items / page_size) + (num_items % page_size > 0) as u64
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

                let condition = Self::build_filter_condition(filters);
                let select = select.filter(condition);

                select
            }

            #(#function_quotes)*
        }

        impl #name {
            pub fn get_db<'a>() -> &'a DbConn {
                let db: &DbConn = DB_READ.get().expect("DB_WRITE is not initialized");
                db
            }
        }

        impl QueryManager<ActiveModel, ModelOptionDto> for #name {

            #get_by_id_quote

            #[tracing::instrument]
            async fn filter(
                pagination: &Pagination,
                order: &Order,
                filter: &Vec<FilterEnum>,
            ) -> Result<QueryResult<ModelOptionDto>, DbErr> {
                let page_size = pagination.page_size.unwrap_or(1);
                let mut page = pagination.page.unwrap_or(1);
                if page <= 0 {
                    page = 1;
                }
                let paginator = Self::build_query(order, filter).paginate(Self::get_db(), page_size);
                let num_pages = paginator.num_pages().await?;
                let result = paginator.fetch_page(page - 1).await?;
                let result: Vec<ModelOptionDto> = result.into_iter().map(|m| m.into()).collect();

                let page_result = QueryResult {
                    total_page: num_pages,
                    result: result,
                };
                Ok(page_result)
            }

            #related_entity_trait_quote

        }

    };

    TokenStream::from(expanded)
}
