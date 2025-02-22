use proc_macro::TokenStream;
use quote::{format_ident, quote};

use syn::{parse_macro_input, DeriveInput};

pub fn response_json(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
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



pub fn response_json_generic(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
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
