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
        use dn_consul::{Consul, GetServiceNodesRequest};
        use opentelemetry::{baggage::BaggageExt, Context};
        use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
        use reqwest::{Client, Method, header::{HeaderName, HeaderValue, HeaderMap}};
        use serde_json::Value;
        use std::collections::HashMap;
        use std::sync::{Arc, LazyLock, RwLock, Mutex};
        use tracing::{debug, error};

        use shared_shared_data_core::roundrobin::RoundRobin;

        type Host = (String, u16);

        // The Global Routing Table
        static TENANT_ROUTING_TABLE: LazyLock<RwLock<HashMap<String, Mutex<RoundRobin<Host>>>>> =  LazyLock::new(|| RwLock::new(HashMap::new()));

        impl #name {
            fn service_name() -> &'static str {
                stringify!(#remote_name_ident)
            }

            fn http_protocol() -> String {
                match std::env::var("HTTP_PROTOCOL") {
                    Ok(val) => val,
                    Err(_) => "http".to_string(),
                }
            }

            pub async fn update_remote(consul: &Consul) {

                let request = GetServiceNodesRequest {
                    service: Self::service_name(),
                    ..Default::default()
                };
                let data = consul.get_service_nodes(request, None).await;
                if let Err(e) = data {
                    debug!(
                        "Failed to get service nodes for {}: {:?}",
                        Self::service_name(), e
                    );
                    return ();
                }
                let data = data.unwrap();
                let instances: Vec<(String, u16, String)> = data.response.iter()
                .map(|service| {
                    let address = service.service.address.clone();
                    let address = if address.is_empty() {
                        service.node.address.clone()
                    } else {
                        address
                    };
                    let port = service.service.port;
                    let tenant = service.service.meta.get("tenant").cloned().unwrap_or("DEFAULT".to_string());
                    (address, port, tenant)
                })
                .collect();
                let mut grouped: HashMap<String, Vec<Host>> = HashMap::new();
                for (ip, port, tenant) in instances {
                    let entry = grouped.entry(tenant).or_insert_with(Vec::new);
                    entry.push((ip, port));
                }
                if let Ok(mut table) = TENANT_ROUTING_TABLE.write() {
                    for (tenant, hosts) in grouped {
                        table.insert(tenant, Mutex::new(RoundRobin::new(hosts)));
                    }
                }
                debug!("Discovered table: {:?}", TENANT_ROUTING_TABLE);

            }

            #[tracing::instrument(name = "call_api", skip(json_body, headers_hashmap), fields(service_name = %Self::service_name()))]
            async fn call_api(
                endpoint: String,
                method: Method,
                json_body: Option<Value>,
                headers_hashmap: HashMap<String, String>,
            ) -> Result<Value, String>
            {
                debug!("Calling API: {} with method {}", endpoint, method);

                let client: ClientWithMiddleware = ClientBuilder::new(Client::new())
                    .with(RequestTracingMiddleware)
                    .build();

                let tenant = Context::current()
                    .baggage()
                    .get("tenant_id")
                    .map(|v| v.to_string())
                    .unwrap_or_else(|| "DEFAULT".to_string());
                debug!("Extracted tenant_id from baggage: {}", tenant);

                let instance = Self::get_instance(tenant.as_str());
                if instance.is_none() {
                    let err_msg = "No available service instances".to_string();
                    error!("{}", err_msg);
                    return Err(err_msg);
                }
                let (ip, port) = instance.unwrap();
                let http_protocol = Self::http_protocol();
                let url = format!("{http_protocol}://{ip}:{port}{endpoint}");
                debug!("Request URL: {}", url);

                let res = match method {
                    Method::POST => {
                        client
                            .post(&url)
                    }
                    Method::PATCH => {
                        client
                            .patch(&url)
                    }
                    Method::GET => {
                        client
                            .get(&url)
                    }
                    _ => {
                        let err_msg = format!("Unsupported HTTP method: {}", method);
                        return Err(err_msg);
                    }
                };



                let mut header_map = HeaderMap::new();

                for (key, value) in headers_hashmap {
                    let header_name = HeaderName::from_bytes(key.as_bytes());
                    let header_value = HeaderValue::from_bytes(value.as_bytes());
                    if header_name.is_err() || header_value.is_err() {
                        continue;
                    }
                    let header_name = header_name.unwrap();
                    let header_value = header_value.unwrap();
                    header_map.insert(header_name, header_value);
                }

                let res = res
                    .header("Content-Type", "application/json")
                    .headers(header_map)
                    .json(&json_body)
                    .send()
                    .await;
                if let Err(e) = res {
                    let err_msg = format!("Failed to send request {}", e);
                    return Err(err_msg);
                }
                let res = res.unwrap();
                if !res.status().is_success() {
                    let err_msg = format!("Return failed status: {}", res.status());
                    return Err(err_msg);
                }
                let body = res.text().await;
                if body.is_err() {
                    let err_msg = format!("Failed to read response body: {}", body.err().unwrap());
                    return Err(err_msg);
                }
                let body = body.unwrap();
                let data = serde_json::from_str::<serde_json::Value>(&body);
                if data.is_err() {
                    let err_msg = format!("Failed to parse response body: {}", data.err().unwrap());
                    return Err(err_msg);
                }
                let data = data.unwrap();
                let data = data.get("data").unwrap().clone();
                Ok(data)
            }

            fn get_instance(tenant_id: &str) -> Option<Host> {
                if let Ok(routing_table) = TENANT_ROUTING_TABLE.read() {
                    if let Some(rr_mutex) = routing_table.get(tenant_id) {
                        if let Ok(mut rr) = rr_mutex.lock() {
                            let result = rr.next_value();
                            return Some(result.clone());
                        }
                    }
                }
                None
            }
        }
    };
    gen.into()
}
