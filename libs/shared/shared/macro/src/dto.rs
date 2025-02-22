use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, punctuated::Punctuated, Data, DeriveInput, Fields, Meta, Token};

pub fn derive_dto(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let mut bto_data: Vec<(String, Vec<String>, bool)> = Vec::new();

    for attr in &input.attrs {
        if attr.path().is_ident("dto") {
            let mut dto_name: String = String::new();
            let mut dto_columns: Vec<String> = Vec::new();
            let mut dto_option: bool = false;
            let nested = attr
                .parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)
                .unwrap();
            for meta in nested {
                match meta {
                    Meta::List(meta_list) => {
                        let name = meta_list.path.get_ident().unwrap();
                        let tokens = meta_list.tokens.clone();
                        if name == "name" {
                            let name = tokens.to_string();
                            dto_name = name.to_owned();
                        } else if name == "columns" {
                            let columns = tokens.to_string();
                            dto_columns =
                                columns.split(",").map(|s| s.trim().to_string()).collect();
                        }
                    }
                    Meta::Path(meta_path) => {
                        let name = meta_path.get_ident().unwrap();
                        if name == "option" {
                            dto_option = true;
                        } else {
                            panic!("Unknown attribute: {}", name);
                        }
                    }
                    _ => {}
                }
            }
            bto_data.push((dto_name.clone(), dto_columns.clone(), dto_option));
        }
    }

    let fields = match &input.data {
        Data::Struct(data_struct) => match &data_struct.fields {
            Fields::Named(fields_named) => &fields_named.named,
            _ => panic!("This macro only works on structs with named fields"),
        },
        _ => panic!("This macro only works on structs"),
    };

    let option_fields = fields.iter()
    // .filter(|field| {
    //     field.ident.as_ref().unwrap().to_string() != "id"
    // })
    .map(|field| {
        let name = &field.ident;
        let ty = &field.ty;
        quote! { pub #name: Option<#ty> }
    });

    let model_option_quotes = fields.iter().map(|field| {
        let name = &field.ident;
        quote! {
            #name: Some(model.#name)
        }
    });

    let option_struct_name = format_ident!("{}OptionDto", name);

    let bto_quotes = bto_data.iter().map(|(dto_name, dto_columns, dto_option)| {
        let dto_name_ident = format_ident!("{}", dto_name);
        let dto_columns_ident: Vec<_> = dto_columns
            .iter()
            .map(|col| format_ident!("{}", col))
            .collect();

        // let struct_name = format_ident!("{}{}", name, dto_name_ident);
        let struct_name = format_ident!("{}Dto", dto_name_ident);

        let field_quotes = dto_columns_ident.iter().map(|col| {
            fields
                .iter()
                .find(|field| {
                    let field_name = field.ident.as_ref().unwrap().to_string();
                    field_name == col.to_string()
                })
                .map(|field| {
                    let name = &field.ident;
                    let ty = &field.ty;
                    if *dto_option {
                        return quote! { pub #name: Option<#ty> };
                    }
                    quote! { pub #name: #ty }
                })
                .unwrap_or_else(|| {
                    panic!("Field {} not found in struct {}", col, name);
                })
        });

        let from_origin_quote = if *dto_option {
            quote! {
                #(#dto_columns_ident: Some(original.#dto_columns_ident),)*
            }
        } else {
            quote! {
                #(#dto_columns_ident: original.#dto_columns_ident,)*
            }
        };

        let from_origin_option_quote = if *dto_option {
            quote! {
                #(#dto_columns_ident: original.#dto_columns_ident,)*
            }
        } else {
            quote! {
                #(#dto_columns_ident: original.#dto_columns_ident.unwrap_or_default(),)*
            }
        };

        let from_new_struct_quote = if *dto_option {
            quote! {
                #(#dto_columns_ident: dto.#dto_columns_ident.unwrap_or_default(),)*
            }
        } else {
            quote! {
                #(#dto_columns_ident: dto.#dto_columns_ident,)*
            }
        };

        let from_new_struct_option_quote = if *dto_option {
            quote! {
                #(#dto_columns_ident: dto.#dto_columns_ident,)*
            }
        } else {
            quote! {
                #(#dto_columns_ident: Some(dto.#dto_columns_ident),)*
            }
        };

        quote! {
            #[derive(Default, Debug, Serialize)]
            pub struct #struct_name {
                #(#field_quotes,)*
            }

            /// Conversion from the original struct to the DTO struct
            impl From<#name> for #struct_name {
                fn from(original: #name) -> Self {
                    Self {
                        #from_origin_quote
                    }
                }
            }

            /// Convert from option_struct_name to the Dto struct
            impl From<#option_struct_name> for #struct_name {
                fn from(original: #option_struct_name) -> Self {
                    Self {
                        #from_origin_option_quote
                    }
                }
            }

            /// Conversion from the DTO struct to the original struct
            impl From<#struct_name> for #name {
                fn from(dto: #struct_name) -> Self {
                    Self {
                        #from_new_struct_quote
                        ..Default::default()
                    }
                }
            }


            /// Convert from the DTO struct to the option struct
            impl From<#struct_name> for #option_struct_name {
                fn from(dto: #struct_name) -> Self {
                    Self {
                        #from_new_struct_option_quote
                        ..Default::default()
                    }
                }
            }


        }
    });

    let expanded = quote! {
        #(#bto_quotes)*

        #[derive(Default, Debug, Serialize)]
        pub struct #option_struct_name {
            #(#option_fields,)*
        }

        impl From<#name> for #option_struct_name {
            fn from(model: #name) -> Self {
                Self {
                    // #from_new_struct_option_quote
                    #(#model_option_quotes,)*
                    ..Default::default()
                }
            }
        }
    };

    TokenStream::from(expanded)
}
