use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Data, DeriveInput, Fields, Ident, Token,
};

mod kw {
    syn::custom_keyword!(name);
    syn::custom_keyword!(columns);
    syn::custom_keyword!(option);
}

struct DtoAttr {
    name: String,
    columns: Vec<String>,
    option: bool,
}

impl Parse for DtoAttr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut name = String::new();
        let mut columns = Vec::new();
        let mut option = false;

        while !input.is_empty() {
            let lookahead = input.lookahead1();
            if lookahead.peek(kw::name) {
                input.parse::<kw::name>()?;
                let content;
                syn::parenthesized!(content in input);
                let tokens: proc_macro2::TokenStream = content.parse()?;
                name = tokens.to_string();
            } else if lookahead.peek(kw::columns) {
                input.parse::<kw::columns>()?;
                let content;
                syn::parenthesized!(content in input);
                let idents = Punctuated::<Ident, Token![,]>::parse_terminated(&content)?;
                columns = idents.iter().map(|i| i.to_string()).collect();
            } else if lookahead.peek(kw::option) {
                input.parse::<kw::option>()?;
                option = true;
            } else {
                return Err(lookahead.error());
            }
            let _ = input.parse::<Token![,]>();
        }

        Ok(DtoAttr {
            name,
            columns,
            option,
        })
    }
}

pub(crate) struct DtoDef {
    pub name: String,
    pub columns: Vec<String>,
    pub option: bool,
}

pub(crate) struct DtoInput {
    pub name: Ident,
    pub dto_defs: Vec<DtoDef>,
    pub fields: Punctuated<syn::Field, Token![,]>,
}

impl DtoInput {
    pub fn parse_from(input: DeriveInput) -> Self {
        let name = input.ident;
        let mut dto_defs = Vec::new();

        for attr in &input.attrs {
            if attr.path().is_ident("dto") {
                let parsed: DtoAttr = attr.parse_args().unwrap();
                dto_defs.push(DtoDef {
                    name: parsed.name,
                    columns: parsed.columns,
                    option: parsed.option,
                });
            }
        }

        let fields = match input.data {
            Data::Struct(data_struct) => match data_struct.fields {
                Fields::Named(fields_named) => fields_named.named,
                _ => panic!("This macro only works on structs with named fields"),
            },
            _ => panic!("This macro only works on structs"),
        };

        DtoInput {
            name,
            dto_defs,
            fields,
        }
    }
}

pub fn derive_dto(input: DtoInput) -> TokenStream {
    let DtoInput {
        name,
        dto_defs,
        fields,
    } = input;

    let option_fields = fields
        .iter()
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

    let bto_quotes = dto_defs.iter().map(|def| {
        let dto_name = &def.name;
        let dto_columns = &def.columns;
        let dto_option = def.option;
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
                    if dto_option {
                        return quote! { pub #name: Option<#ty> };
                    }
                    quote! { pub #name: #ty }
                })
                .unwrap_or_else(|| {
                    panic!("Field {} not found in struct {}", col, name);
                })
        });

        let from_origin_quote = if dto_option {
            quote! {
                #(#dto_columns_ident: Some(original.#dto_columns_ident),)*
            }
        } else {
            quote! {
                #(#dto_columns_ident: original.#dto_columns_ident,)*
            }
        };

        let from_origin_option_quote = if dto_option {
            quote! {
                #(#dto_columns_ident: original.#dto_columns_ident,)*
            }
        } else {
            quote! {
                #(#dto_columns_ident: original.#dto_columns_ident.unwrap_or_default(),)*
            }
        };

        let from_new_struct_quote = if dto_option {
            quote! {
                #(#dto_columns_ident: dto.#dto_columns_ident.unwrap_or_default(),)*
            }
        } else {
            quote! {
                #(#dto_columns_ident: dto.#dto_columns_ident,)*
            }
        };

        let from_new_struct_option_quote = if dto_option {
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
