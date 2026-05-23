use serde::Serialize;
use utoipa::{
    openapi::{
        security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
        ServerBuilder,
    },
    Modify, ToSchema,
};

/// Error detail information
#[derive(Serialize, ToSchema)]
pub struct ErrorDetail {
    pub error_type: Option<String>,
    pub details: Option<serde_json::Value>,
}

/// Standard error response
#[derive(Serialize, ToSchema)]
pub struct ErrorResponse {
    pub status: i32,
    pub data: ErrorDetail,
}

pub struct ServerAddon;

impl Modify for ServerAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let url = std::env::var("SERVER_URL").unwrap_or_else(|_| "/".to_string());
        openapi.servers = Some(vec![ServerBuilder::new().url(url).build()]);
    }
}

pub struct JwtSecurityAddon;

impl Modify for JwtSecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.as_mut().unwrap();
        components.add_security_scheme(
            "token",
            SecurityScheme::Http(
                HttpBuilder::new()
                    .scheme(HttpAuthScheme::Bearer)
                    .bearer_format("JWT")
                    .build(),
            ),
        );
    }
}
