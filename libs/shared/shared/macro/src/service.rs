use proc_macro::TokenStream;
use quote::quote;

use syn::{parse_macro_input, punctuated::Punctuated, DeriveInput, Meta, Token};

pub fn remote_service(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;

    let mut remote_name: String = String::new();
    for attr in &ast.attrs {
        if attr.path().is_ident("remote") {
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
                            remote_name = name.to_owned();
                        }
                    }

                    _ => {}
                }
            }
        }
    }
    let remote_name_ident = syn::Ident::new(&remote_name, name.span());
    let gen = quote! {
        static IP_PORTS: LazyLock<Mutex<RoundRobin<(String, u16)>>> = LazyLock::new(|| Mutex::new(RoundRobin::new(vec![])));
        impl #name {
            pub fn service_name() -> &'static str {
                stringify!(#remote_name_ident)
            }

            async fn get_ip_ports<'a>(consul: &'a Consul) -> Vec<(String, u16)> {
                let ip_ports = consul.get_service_addresses_and_ports(Self::service_name(), None);
                ip_ports.await.unwrap_or_default()
            }

            pub async fn update_remote(consul: &Consul) {
                let ip_ports: Vec<(String, u16)> = Self::get_ip_ports(consul).await;
                debug!("Discovered auth service instances: {:?}", ip_ports);
                if let Ok(mut rr) = IP_PORTS.lock() {
                    rr.replace_values(ip_ports);
                }
            }

            pub fn get_instance() -> Option<(String, u16)> {
                if let Ok(mut rr) = IP_PORTS.lock() {
                    let result = rr.next_value();
                    Some(result.clone())
                } else {
                    None
                }
            }
        }
    };
    gen.into()

    /*
    let input = parse_macro_input!(item as ItemStruct);

    // Define the new method
    let new_method = quote! {
        pub fn new_method(&self) {
            println!("Hello from a new method!");
        }
    };

    // Get the struct's identifier
    let struct_name = &input.ident;

    // Construct the output TokenStream, including the original struct and the new impl block
    quote! {
        #input // The original struct definition
        impl #struct_name {
            #new_method // The added method
        }
    }
    .into()

    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    // let struct_name = format_ident!("{}Response", name);

    let new_method = quote! {
        pub fn new_method(&self) {
            println!("Hello from a new method!");
        }
    };

    let output = quote! {
        #[derive(ToSchema)]
        impl  #name {
            #new_method
        }
    };

    output.into()
    */
}
