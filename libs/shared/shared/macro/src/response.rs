use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{DeriveInput, Ident};

pub(crate) struct ResponseInput {
    pub name: Ident,
}

impl ResponseInput {
    pub fn parse_from(input: DeriveInput) -> Self {
        ResponseInput { name: input.ident }
    }
}

pub fn response_json(input: ResponseInput) -> TokenStream {
    let name = &input.name;
    let struct_name = format_ident!("{}Response", name);

    let output = quote! {
        #[derive(ToSchema)]
        pub struct #struct_name {
            pub status: i32,
            pub data: #name
        }
    };

    output.into()
}

pub fn response_json_generic(input: ResponseInput) -> TokenStream {
    let name = &input.name;
    let struct_name = format_ident!("{}Response", name);

    let output = quote! {
        #[derive(ToSchema)]
        pub struct #struct_name<T> {
            pub status: i32,
            pub data: #name<T>
        }
    };

    output.into()
}
