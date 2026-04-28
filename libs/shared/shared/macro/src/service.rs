use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    DeriveInput, Ident,
};

mod kw {
    syn::custom_keyword!(name);
}

struct RemoteAttr {
    name: String,
}

impl Parse for RemoteAttr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<kw::name>()?;
        let content;
        syn::parenthesized!(content in input);
        let name_tokens: proc_macro2::TokenStream = content.parse()?;
        Ok(RemoteAttr {
            name: name_tokens.to_string(),
        })
    }
}

pub(crate) struct RemoteServiceInput {
    pub name: Ident,
    pub remote_name: String,
}

impl RemoteServiceInput {
    pub fn parse_from(input: DeriveInput) -> Self {
        let name = input.ident;
        let mut remote_name = String::new();
        for attr in &input.attrs {
            if attr.path().is_ident("remote") {
                let parsed: RemoteAttr = attr.parse_args().unwrap();
                remote_name = parsed.name;
            }
        }
        RemoteServiceInput { name, remote_name }
    }
}

pub fn remote_service(input: RemoteServiceInput) -> TokenStream {
    let RemoteServiceInput { name, remote_name } = input;
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
                    .with(shared_shared_middleware::RequestTracingMiddleware)
                    .build();

                let tenant = Context::current()
                    .baggage()
                    .get("tenant_id")
                    .map(|v| v.to_string())
                    .unwrap_or_else(|| "DEFAULT".to_string());
                debug!("Extracted tenant_id from baggage: {}", tenant);

                let (ip, port) = Self::get_instance(tenant.as_str()).ok_or_else(|| {
                    error!("No available service instances");
                    "No available service instances".to_string()
                })?;

                let url = format!("{}://{}:{}{}", Self::http_protocol(), ip, port, endpoint);
                debug!("Request URL: {}", url);

                let builder = match method {
                    Method::POST => client.post(&url),
                    Method::PATCH => client.patch(&url),
                    Method::GET => client.get(&url),
                    _ => return Err(format!("Unsupported HTTP method: {}", method)),
                };

                let mut header_map = HeaderMap::new();
                for (key, value) in headers_hashmap {
                    if let (Ok(name), Ok(val)) = (
                        HeaderName::from_bytes(key.as_bytes()),
                        HeaderValue::from_bytes(value.as_bytes()),
                    ) {
                        header_map.insert(name, val);
                    }
                }

                let res = builder
                    .header("Content-Type", "application/json")
                    .headers(header_map)
                    .json(&json_body)
                    .send()
                    .await
                    .map_err(|e| format!("Failed to send request: {}", e))?;

                if !res.status().is_success() {
                    return Err(format!("Return failed status: {}", res.status()));
                }

                let body = res.text().await
                    .map_err(|e| format!("Failed to read response body: {}", e))?;

                let data: Value = serde_json::from_str(&body)
                    .map_err(|e| format!("Failed to parse response body: {}", e))?;

                data.get("data")
                    .cloned()
                    .ok_or_else(|| "Response missing 'data' field".to_string())
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
