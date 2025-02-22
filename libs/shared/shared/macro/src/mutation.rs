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
                async fn create_uuid(db: &DbConn, model: Model) -> Result<Uuid, DbErr> {
                    let mut active_model: ActiveModel = model.into();
                    active_model.not_set(Column::Id);
                    let result_model = active_model.insert(db).await;
                    let id = result_model?.id;
                    Ok(id)
                }

                async fn create_i32(db: &DbConn, model: Model) -> Result<i32, DbErr> {
                    unimplemented!("Not implemented")
                }
            }
        }
        "i32" => quote! {
            async fn create_uuid(db: &DbConn, model: Model) -> Result<Uuid, DbErr> {
                unimplemented!("Not implemented")
            }

            async fn create_i32(db: &DbConn, model: Model) -> Result<i32, DbErr> {
                let mut active_model: ActiveModel = model.into();
                active_model.not_set(Column::Id);
                let result_model = active_model.insert(db).await;
                let id = result_model?.id;
                Ok(id)
            }
        },
        _ => panic!("Unsupported key type: {}", key_type_str),
    };

    let upate_quote = match key_type_str.as_str() {
        "Uuid" => {
            quote! {
                async fn update_by_id_uuid(
                    db: &DbConn,
                    id: Uuid,
                    model_option: ModelOptionDto,
                ) -> Result<bool, DbErr> {
                    let exists = Entity::find_by_id(id)
                        .one(db)
                        .await?
                        .ok_or(DbErr::RecordNotFound("Not found".to_string()))?;

                    let active_model = assign(exists.into(), model_option);
                    active_model.update(db).await?;

                    Ok(true)
                }

                async fn update_by_id_i32(
                    db: &DbConn,
                    id: i32,
                    model_option: ModelOptionDto,
                ) -> Result<bool, DbErr> {
                    unimplemented!("Not implemented")
                }
            }
        }
        "i32" => quote! {
            async fn update_by_id_uuid(
                db: &DbConn,
                id: Uuid,
                model_option: ModelOptionDto,
            ) -> Result<bool, DbErr> {
                unimplemented!("Not implemented")
            }

            async fn update_by_id_i32(
                db: &DbConn,
                id: i32,
                model_option: ModelOptionDto,
            ) -> Result<bool, DbErr> {
                let exists = Entity::find_by_id(id)
                .one(db)
                .await?
                .ok_or(DbErr::RecordNotFound("Not found".to_string()))?;

                let active_model = assign(exists.into(), model_option);
                active_model.update(db).await?;

                Ok(true)            }
        },
        _ => panic!("Unsupported key type: {}", key_type_str),
    };

    let delete_quote = match key_type_str.as_str() {
        "Uuid" => {
            quote! {
                async fn delete_by_id_uuid(db: &DbConn, id: Uuid) -> Result<bool, DbErr> {
                    let model: ActiveModel = Entity::find_by_id(id)
                        .one(db)
                        .await?
                        .ok_or(DbErr::RecordNotFound("Not found".to_string()))
                        .map(Into::into)?;
    
                    model.delete(db).await?;
    
                    Ok(true)
                }
    
                async fn delete_by_id_i32(db: &DbConn, id: i32) -> Result<bool, DbErr> {
                    unimplemented!("Not implemented")
                }
            }
        }
        "i32" => quote! {
            async fn delete_by_id_uuid(db: &DbConn, id: Uuid) -> Result<bool, DbErr> {
                unimplemented!("Not implemented")
            }

            async fn delete_by_id_i32(db: &DbConn, id: i32) -> Result<bool, DbErr> {
                let model: ActiveModel = Entity::find_by_id(id)
                    .one(db)
                    .await?
                    .ok_or(DbErr::RecordNotFound("Not found".to_string()))
                    .map(Into::into)?;

                model.delete(db).await?;

                Ok(true)   
            }

        },
        _ => panic!("Unsupported key type: {}", key_type_str),
    };

    let expanded = quote! {
        use sea_orm::{entity::ActiveModelTrait, EntityTrait};
        use shared_shared_data_core::mutation::MutationManager;

        impl MutationManager<ActiveModel, Model, ModelOptionDto> for #name {
            #create_quote
            #upate_quote
            #delete_quote
        }
    };
    expanded.into()
}
