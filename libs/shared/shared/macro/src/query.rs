use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    parse::{Parse, ParseStream},
    DeriveInput, Ident, LitStr, Token,
};

mod kw {
    syn::custom_keyword!(key_type);
    syn::custom_keyword!(column_name);
    syn::custom_keyword!(entity);
    syn::custom_keyword!(column);
    syn::custom_keyword!(field);
    syn::custom_keyword!(name);
}

struct QueryAttr {
    key_type: proc_macro2::TokenStream,
}

impl Parse for QueryAttr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<kw::key_type>()?;
        let content;
        syn::parenthesized!(content in input);
        let key_type: proc_macro2::TokenStream = content.parse()?;
        Ok(QueryAttr { key_type })
    }
}

struct QueryFilterAttr {
    column_name: proc_macro2::TokenStream,
}

impl Parse for QueryFilterAttr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<kw::column_name>()?;
        let content;
        syn::parenthesized!(content in input);
        let column_name: proc_macro2::TokenStream = content.parse()?;
        Ok(QueryFilterAttr { column_name })
    }
}

struct QueryRelatedAttr {
    entity_tokens: proc_macro2::TokenStream,
    column_tokens: Option<proc_macro2::TokenStream>,
    field_name: String,
    include_name: Option<String>,
}

impl Parse for QueryRelatedAttr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut entity_tokens = None;
        let mut column_tokens = None;
        let mut field_name = None;
        let mut include_name = None;

        while !input.is_empty() {
            let lookahead = input.lookahead1();
            if lookahead.peek(kw::entity) {
                input.parse::<kw::entity>()?;
                let content;
                syn::parenthesized!(content in input);
                entity_tokens = Some(content.parse::<proc_macro2::TokenStream>()?);
            } else if lookahead.peek(kw::column) {
                input.parse::<kw::column>()?;
                let content;
                syn::parenthesized!(content in input);
                column_tokens = Some(content.parse::<proc_macro2::TokenStream>()?);
            } else if lookahead.peek(kw::field) {
                input.parse::<kw::field>()?;
                let content;
                syn::parenthesized!(content in input);
                field_name = Some(content.parse::<Ident>()?.to_string());
            } else if lookahead.peek(kw::name) {
                input.parse::<kw::name>()?;
                let content;
                syn::parenthesized!(content in input);
                include_name = Some(content.parse::<LitStr>()?.value());
            } else {
                return Err(lookahead.error());
            }
            let _ = input.parse::<Token![,]>();
        }

        Ok(QueryRelatedAttr {
            entity_tokens: entity_tokens
                .ok_or_else(|| input.error("missing `entity(...)` in query_related"))?,
            column_tokens,
            field_name: field_name
                .ok_or_else(|| input.error("missing `field(...)` in query_related"))?,
            include_name,
        })
    }
}

pub(crate) struct RelatedEntityDef {
    pub entity_tokens: proc_macro2::TokenStream,
    pub column_tokens: Option<proc_macro2::TokenStream>,
    pub field_name: String,
    pub include_name: String,
}

pub(crate) struct QueryInput {
    pub name: Ident,
    pub key_type_str: String,
    pub filter_columns: Vec<String>,
    pub related_entities: Vec<RelatedEntityDef>,
}

impl QueryInput {
    pub fn parse_from(input: DeriveInput) -> Self {
        let name = input.ident;
        let mut key_type_str = String::new();
        let mut filter_columns: Vec<String> = Vec::new();
        let mut related_entities: Vec<RelatedEntityDef> = Vec::new();

        for attr in &input.attrs {
            if attr.path().is_ident("query") {
                let parsed: QueryAttr = attr.parse_args().unwrap();
                key_type_str = parsed.key_type.to_string();
            } else if attr.path().is_ident("query_filter") {
                let parsed: QueryFilterAttr = attr.parse_args().unwrap();
                filter_columns.push(parsed.column_name.to_string());
            } else if attr.path().is_ident("query_related") {
                let parsed: QueryRelatedAttr = attr.parse_args().unwrap();
                let inc = parsed
                    .include_name
                    .unwrap_or_else(|| parsed.field_name.clone());
                related_entities.push(RelatedEntityDef {
                    entity_tokens: parsed.entity_tokens,
                    column_tokens: parsed.column_tokens,
                    field_name: parsed.field_name,
                    include_name: inc,
                });
            }
        }

        // Also register related entity columns as filter columns
        for rel in &related_entities {
            if let Some(col_tok) = &rel.column_tokens {
                let col_str = col_tok.to_string();
                if !filter_columns.contains(&col_str) {
                    filter_columns.push(col_str);
                }
            }
        }

        QueryInput {
            name,
            key_type_str,
            filter_columns,
            related_entities,
        }
    }
}

pub fn query_impl(input: QueryInput) -> TokenStream {
    let QueryInput {
        name,
        key_type_str,
        filter_columns,
        related_entities,
    } = input;

    let function_quotes = filter_columns.iter().map(|column_name| {
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

        if let Some(col_tok) = &rel.column_tokens {
            let col_str = col_tok.to_string();
            let filter_fn = format_ident!("filter_condition_{}", col_str.to_lowercase());
            quote! {
                if includes.iter().any(|s| s == #inc_name) {
                    let prefix = format!("{}.", #inc_name);
                    let rel_filters: Vec<&FilterEnum> = related_filters.iter()
                        .filter(|f| f.get_name().starts_with(&prefix))
                        .collect();
                    let mut query = parent_model.find_related(#entity_tok);
                    for rf in &rel_filters {
                        let col_name = rf.get_name().strip_prefix(&prefix).unwrap_or("").to_string();
                        if let Ok(col) = #col_tok::from_str(&col_name) {
                            query = query.filter(Self::#filter_fn(col, rf));
                        }
                    }
                    let related = query.all(Self::get_db()).await?;
                    model.#field_ident = Some(related.into_iter().map(|r| r.into()).collect());
                }
            }
        } else {
            quote! {
                if includes.iter().any(|s| s == #inc_name) {
                    let related = parent_model
                        .find_related(#entity_tok)
                        .all(Self::get_db())
                        .await?;
                    model.#field_ident = Some(related.into_iter().map(|r| r.into()).collect());
                }
            }
        }
    }).collect();

    // Generate per-relation loading blocks, each gated by includes check
    let related_load_blocks: Vec<_> = related_entities.iter().map(|rel| {
        let entity_tok = &rel.entity_tokens;
        let field_ident = format_ident!("{}", rel.field_name);
        let inc_name = &rel.include_name;

        if let Some(col_tok) = &rel.column_tokens {
            let col_str = col_tok.to_string();
            let filter_fn = format_ident!("filter_condition_{}", col_str.to_lowercase());
            quote! {
                if includes.iter().any(|s| s == #inc_name) {
                    let prefix = format!("{}.", #inc_name);
                    let rel_filters: Vec<&FilterEnum> = related_filters.iter()
                        .filter(|f| f.get_name().starts_with(&prefix))
                        .collect();

                    let mut base_query = Entity::find()
                        .filter(Column::Id.is_in(parent_ids.clone()))
                        .find_with_related(#entity_tok);

                    for rf in &rel_filters {
                        let col_name = rf.get_name().strip_prefix(&prefix).unwrap_or("").to_string();
                        if let Ok(col) = #col_tok::from_str(&col_name) {
                            base_query = base_query.filter(Self::#filter_fn(col, rf));
                        }
                    }

                    let with_related = base_query.all(Self::get_db()).await?;
                    for (parent, related) in with_related {
                        if let Some(dto) = result_map.get_mut(&parent.id) {
                            dto.#field_ident = Some(related.into_iter().map(|r| r.into()).collect());
                        }
                    }
                }
            }
        } else {
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
        }
    }).collect();

    let related_entity_trait_quote = if !related_load_blocks.is_empty() {
        let get_by_id_with_related_quote = match key_type_str.as_str() {
            "Uuid" => quote! {
                #[tracing::instrument]
                async fn get_by_id_uuid_with_related_entities(
                    id: Uuid,
                    includes: &Vec<String>,
                    related_filters: &Vec<FilterEnum>,
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
                #[tracing::instrument]
                async fn get_by_id_i32_with_related_entities(
                    id: i32,
                    includes: &Vec<String>,
                    related_filters: &Vec<FilterEnum>,
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
                #[tracing::instrument]
                async fn get_by_id_str_with_related_entities(
                    id: String,
                    includes: &Vec<String>,
                    related_filters: &Vec<FilterEnum>,
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
            #[tracing::instrument]
            async fn filter_with_related_entities(
                pagination: &Pagination,
                order: &Order,
                filter: &Vec<FilterEnum>,
                includes: &Vec<String>,
                related_filters: &Vec<FilterEnum>,
            ) -> Result<QueryResult<ModelOptionDto>, DbErr> {
                if includes.is_empty() {
                    return Self::filter(pagination, order, filter).await;
                }

                let (num_pages, parents) = Self::paginate_query(pagination, order, filter).await?;

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

            async fn paginate_query(
                pagination: &Pagination,
                order: &Order,
                filters: &Vec<FilterEnum>,
            ) -> Result<(u64, Vec<<Entity as EntityTrait>::Model>), DbErr> {
                let page_size = pagination.page_size.unwrap_or(1);
                let page = pagination.page.unwrap_or(1).max(1);
                let paginator = Self::build_query(order, filters).paginate(Self::get_db(), page_size);
                let num_pages = paginator.num_pages().await?;
                let result = paginator.fetch_page(page - 1).await?;
                Ok((num_pages, result))
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
                let (num_pages, result) = Self::paginate_query(pagination, order, filter).await?;
                let result: Vec<ModelOptionDto> = result.into_iter().map(|m| m.into()).collect();

                Ok(QueryResult {
                    total_page: num_pages,
                    result,
                })
            }

            #related_entity_trait_quote

        }

    };

    TokenStream::from(expanded)
}
