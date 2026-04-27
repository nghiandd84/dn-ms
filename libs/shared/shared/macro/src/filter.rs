use proc_macro::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{Data, DeriveInput, Fields, Ident};

const FILTER_TYPES: [&str; 15] = [
    "string",
    "bool",
    "json",
    "i32",
    "i64",
    "i8",
    "u32",
    "u64",
    "f32",
    "f64",
    "uuid",
    "datetime",
    "vecstring",
    "vecuuid",
    "jsonvalue"
];

pub(crate) struct FilterField {
    pub name: Ident,
    pub ty: syn::Type,
}

pub(crate) struct FilterInput {
    pub name: Ident,
    pub fields: Vec<FilterField>,
}

impl FilterInput {
    pub fn parse_from(input: DeriveInput) -> Self {
        let name = input.ident;
        let raw_fields = match input.data {
            Data::Struct(data_struct) => match data_struct.fields {
                Fields::Named(fields_named) => fields_named.named,
                _ => panic!("This macro only works on structs with named fields"),
            },
            _ => panic!("This macro only works on structs"),
        };

        let fields = raw_fields
            .into_iter()
            .filter(|field| {
                !field.attrs.iter().any(|attr| attr.path().is_ident("skip_param"))
            })
            .map(|field| {
                let name = field.ident.clone().unwrap();
                let ty = match &field.ty {
                    syn::Type::Path(type_path)
                        if type_path
                            .path
                            .segments
                            .last()
                            .is_some_and(|seg| seg.ident == "Option") =>
                    {
                        if let syn::PathArguments::AngleBracketed(args) =
                            &type_path.path.segments.last().unwrap().arguments
                        {
                            if let syn::GenericArgument::Type(inner_ty) = args.args.first().unwrap() {
                                inner_ty.clone()
                            } else {
                                field.ty.clone()
                            }
                        } else {
                            field.ty.clone()
                        }
                    }
                    _ => field.ty.clone(),
                };
                FilterField { name, ty }
            })
            .collect();

        FilterInput { name, fields }
    }
}

pub fn filter_macro_derive_impl(input: FilterInput) -> TokenStream {
    let FilterInput { name, fields } = input;
    let param_filter_name = format_ident!("{}FilterParams", name);

    let builder_fields = fields.iter().map(|f| {
        let name = &f.name;
        let ty = &f.ty;
        let ty_str = ty.to_token_stream().to_string();
        let ty_str = ty_str.replace(['<', '>'], "");
        let ty_str = ty_str.replace(' ', "");
        let ty_str_lower = ty_str.to_lowercase();
        if !FILTER_TYPES.iter().any(|&x| x == ty_str_lower) {
            let param_filter_name = format_ident!("{}FilterParams", ty_str);
            quote! {
                pub #name: Option<#param_filter_name>
            }
        } else {
            let default_str = format!("default_none_{}", ty_str_lower).to_string();
            let default_quote = quote! {#default_str};
            let deserialize_str = format!("deserialize_filter_from_{}", ty_str_lower).to_string();
            let deserialize_quote = quote! {#deserialize_str};
            quote! {
                #[serde(
                    default = #default_quote,
                    deserialize_with = #deserialize_quote
                )]
                pub #name: Option<FilterParam<#ty>>
            }
        }
    });

    let builder_all_filters = fields.iter().map(|f| {
        let name = &f.name;
        let ty = &f.ty;
        let field_name_str = name.to_string();
        let field_name_quote = quote! {#field_name_str};
        let ty_str = ty.to_token_stream().to_string();
        let ty_str_lower = ty_str.to_lowercase();
        if !FILTER_TYPES.iter().any(|&x| x == ty_str_lower) {
            quote! {
                if self.#name.is_some() {
                    let mut filters = self.#name.as_ref().unwrap().clone().all_filters();
                    for f in filters.iter_mut() {
                        f.add_name_prefix(#field_name_quote);
                    }
                    result.append(&mut filters);
                }
            }
        } else {
            let enum_type = capitalize(&ty_str);
            let enum_type_quote = format_ident!("{}", enum_type);
            quote! {
                if self.#name.is_some() {
                    let mut filter = self.#name.as_ref().unwrap().clone();
                    filter.name = #field_name_quote.to_owned();
                    let filter_enum = FilterEnum::#enum_type_quote(filter);
                    result.push(filter_enum);
                }
            }
        }
    });

    let gen = quote! {

        #[derive(serde::Deserialize, Debug)]
        pub struct #param_filter_name {
            #(#builder_fields,)*
        }

        impl #param_filter_name {
            pub fn all_filters(self: &Self) -> Vec<FilterEnum>{
                let mut result: Vec<FilterEnum> = vec![];
                #(#builder_all_filters)*
                result
            }
        }

    };
    gen.into()
}

pub fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}
