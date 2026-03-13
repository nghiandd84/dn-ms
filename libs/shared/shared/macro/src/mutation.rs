use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, punctuated::Punctuated, DeriveInput, Meta, Token};

pub fn mutation_impl(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let mut key_type_str: String = String::new();
    for attr in &input.attrs {
        if attr.path().is_ident("mutation") {
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
    }

    let create_quote = match key_type_str.as_str() {
        "Uuid" => {
            quote! {
                #[tracing::instrument]
                async fn create_uuid(model: Model) -> Result<Uuid, DbErr> {
                    let mut active_model: ActiveModel = model.into();
                    active_model.not_set(Column::Id);
                    let result_model = active_model.insert(Self::get_db()).await;
                    let id = result_model?.id;
                    Ok(id)
                }

                #[tracing::instrument]
                async fn bulk_create_uuid(models: Vec<Model>) -> Result<Vec<Uuid>, DbErr> {
                    let active_models: Vec<ActiveModel> = models.into_iter().map(|model| {
                        let mut active_model: ActiveModel = model.into();
                        active_model.not_set(Column::Id);
                        active_model
                    }).collect();

                    let result_models = Entity::insert_many(active_models).exec_with_returning(Self::get_db()).await?;
                    let ids = result_models.iter().map(|model| model.id).collect();
                    Ok(ids)
                }

                async fn create_i32(model: Model) -> Result<i32, DbErr> {
                    unimplemented!("Not implemented")
                }

                async fn bulk_create_i32(models: Vec<Model>) -> Result<Vec<i32>, DbErr> {
                    unimplemented!("Not implemented")
                }

            }
        }
        "i32" => quote! {
            async fn create_uuid(model: Model) -> Result<Uuid, DbErr> {
                unimplemented!("Not implemented")
            }

            async fn bulk_create_uuid(models: Vec<Model>) -> Result<Vec<Uuid>, DbErr> {
                unimplemented!("Not implemented")
            }

            #[tracing::instrument]
            async fn create_i32(model: Model) -> Result<i32, DbErr> {
                let mut active_model: ActiveModel = model.into();
                active_model.not_set(Column::Id);
                let result_model = active_model.insert(Self::get_db()).await;
                let id = result_model?.id;
                Ok(id)
            }

            #[tracing::instrument]
            async fn bulk_create_i32(models: Vec<Model>) -> Result<Vec<i32>, DbErr> {

                let active_models: Vec<ActiveModel> = models.into_iter().map(|model| {
                    let mut active_model: ActiveModel = model.into();
                    active_model.not_set(Column::Id);
                    active_model
                }).collect();

                let result_models = Entity::insert_many(active_models).exec_with_returning(Self::get_db()).await?;
                let ids = result_models.iter().map(|model| model.id).collect();
                Ok(ids)
            }
        },
        _ => panic!("Unsupported key type: {}", key_type_str),
    };

    let upate_quote = match key_type_str.as_str() {
        "Uuid" => {
            quote! {
                #[tracing::instrument]
                async fn update_by_id_uuid(
                    id: Uuid,
                    model_option: ModelOptionDto,
                ) -> Result<bool, DbErr> {
                    let exists = Entity::find_by_id(id)
                        .one(Self::get_db())
                        .await?
                        .ok_or(DbErr::RecordNotFound("Not found".to_string()))?;

                    let active_model = assign(exists.into(), model_option);
                    active_model.update(Self::get_db()).await?;

                    Ok(true)
                }

                #[tracing::instrument]
                async fn bulk_update_by_id_uuid(
                    data: Vec<(Uuid, ModelOptionDto)>,
                ) -> Result<Vec<Uuid>, DbErr> {
                    let mut ids = Vec::new();
                    for (id, model_option) in data {
                        let exists = Entity::find_by_id(id)
                            .one(Self::get_db())
                            .await?
                            .ok_or(DbErr::RecordNotFound("Not found".to_string()))?;

                        let active_model = assign(exists.into(), model_option);
                        active_model.update(Self::get_db()).await?;
                        ids.push(id);
                    }
                    Ok(ids)
                }

                async fn update_by_id_i32(
                    id: i32,
                    model_option: ModelOptionDto,
                ) -> Result<bool, DbErr> {
                    unimplemented!("Not implemented")
                }

                async fn bulk_update_by_id_i32(
                    data: Vec<(i32, ModelOptionDto)>,
                ) -> Result<Vec<i32>, DbErr> {
                    unimplemented!("Not implemented")
                }

            }
        }
        "i32" => quote! {
            async fn update_by_id_uuid(
                id: Uuid,
                model_option: ModelOptionDto,
            ) -> Result<bool, DbErr> {
                unimplemented!("Not implemented")
            }

            async fn bulk_update_by_id_uuid(
                    data: Vec<(Uuid, ModelOptionDto)>,
                ) -> Result<Vec<Uuid>, DbErr> {
                unimplemented!("Not implemented")
            }

            #[tracing::instrument]
            async fn update_by_id_i32(
                id: i32,
                model_option: ModelOptionDto,
            ) -> Result<bool, DbErr> {
                let exists = Entity::find_by_id(id)
                    .one(Self::get_db())
                    .await?
                    .ok_or(DbErr::RecordNotFound("Not found".to_string()))?;

                let active_model = assign(exists.into(), model_option);
                active_model.update(Self::get_db()).await?;

                Ok(true)
            }

            #[tracing::instrument]
            async fn bulk_update_by_id_i32(
                    data: Vec<(i32, ModelOptionDto)>,
                ) -> Result<Vec<i32>, DbErr> {
                let mut ids = Vec::new();
                for (id, model_option) in data {
                    let exists = Entity::find_by_id(id)
                        .one(Self::get_db())
                        .await?
                        .ok_or(DbErr::RecordNotFound("Not found".to_string()))?;

                    let active_model = assign(exists.into(), model_option);
                    active_model.update(Self::get_db()).await?;
                    ids.push(id);
                }
                Ok(ids)
            }
        },
        _ => panic!("Unsupported key type: {}", key_type_str),
    };

    let delete_quote = match key_type_str.as_str() {
        "Uuid" => {
            quote! {
                #[tracing::instrument]
                async fn delete_by_id_uuid(id: Uuid) -> Result<bool, DbErr> {
                    let model: ActiveModel = Entity::find_by_id(id)
                        .one(Self::get_db())
                        .await?
                        .ok_or(DbErr::RecordNotFound("Not found".to_string()))
                        .map(Into::into)?;

                    model.delete(Self::get_db()).await?;

                    Ok(true)
                }

                async fn delete_by_id_i32(id: i32) -> Result<bool, DbErr> {
                    unimplemented!("Not implemented")
                }
            }
        }
        "i32" => quote! {
            async fn delete_by_id_uuid(id: Uuid) -> Result<bool, DbErr> {
                unimplemented!("Not implemented")
            }
            #[tracing::instrument]
            async fn delete_by_id_i32(id: i32) -> Result<bool, DbErr> {
                let model: ActiveModel = Entity::find_by_id(id)
                    .one(Self::get_db())
                    .await?
                    .ok_or(DbErr::RecordNotFound("Not found".to_string()))
                    .map(Into::into)?;

                model.delete(Self::get_db()).await?;

                Ok(true)
            }

        },
        _ => panic!("Unsupported key type: {}", key_type_str),
    };

    let expanded = quote! {
        use sea_orm::{entity::ActiveModelTrait, EntityTrait};
        use shared_shared_data_core::mutation::MutationManager;
        use shared_shared_config::db::DB_WRITE;

        impl #name {
            pub fn get_db<'a>() -> &'a DbConn {
                let db: &DbConn = DB_WRITE.get().expect("DB_WRITE is not initialized");
                db
            }
        }

        impl MutationManager<ActiveModel, Model, ModelOptionDto> for #name {
            #create_quote
            #upate_quote
            #delete_quote
        }
    };
    expanded.into()
}
