use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input,  Data, DeriveInput,  Fields, ItemFn};



pub fn make_answer(_item: TokenStream) -> TokenStream {
    "fn answer() -> u32 { 42 }".parse().unwrap()
}

pub fn greet_macro_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;
    let gen = quote! {
        trait Greet {
            fn greet(&self);
        }
        impl Greet for #name {
            fn greet(&self) {
                println!("Hello, I'm {}!", stringify!(#name));
            }
        }
    };
    gen.into()
}

pub fn log_function_name(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);
    let fn_name = &input_fn.sig.ident;
    let fn_body = &input_fn.block;

    let output = quote! {
        fn #fn_name() {
            println!("Entering function: {}", stringify!(#fn_name));
            #fn_body
        }
    };

    output.into()
}

pub fn derive_builder(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let _attrs = &input.attrs;
   

    let builder_name = format_ident!("{}Builder", name);

    let fields = match &input.data {
        Data::Struct(data_struct) => match &data_struct.fields {
            Fields::Named(fields_named) => &fields_named.named,
            _ => panic!("This macro only works on structs with named fields"),
        },
        _ => panic!("This macro only works on structs"),
    };

    let field_names = fields.iter().map(|field| &field.ident);
    let _field_types = fields.iter().map(|field| &field.ty);

    let builder_fields = fields.iter().map(|field| {
        let name = &field.ident;
        let ty = &field.ty;
        quote! { #name: Option<#ty> }
    });

    let builder_setters = fields.iter().map(|field| {
        let name = &field.ident;
        let ty = &field.ty;
        quote! {
            pub fn #name(&mut self, #name: #ty) -> &mut Self {
                self.#name = Some(#name);
                self
            }
        }
    });

    let builder_build = field_names
        .clone()
        .zip(field_names.clone())
        .map(|(name, name2)| {
            quote! {
                #name: self.#name2.take().ok_or(concat!(stringify!(#name), " is not set"))?
            }
        });

    let expanded = quote! {
        impl #name {
            pub fn builder() -> #builder_name {
                #builder_name {
                    #(#field_names: None,)*
                }
            }
        }

        pub struct #builder_name {
            #(#builder_fields,)*
        }

        impl #builder_name {
            #(#builder_setters)*

            pub fn build(&mut self) -> Result<#name, Box<dyn std::error::Error>> {
                Ok(#name {
                    #(#builder_build,)*
                })
            }
        }
    };

    TokenStream::from(expanded)
}

