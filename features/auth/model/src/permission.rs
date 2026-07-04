use serde::{Deserialize, Serialize};
use shared_shared_macro::{ParamFilter, Response};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

use features_auth_entities::permission::{
    Model, ModelOptionDto, PermissionForCreateDto, PermissionForCreateRequestDto,
};

fn validate_resource_format(resource: &str) -> Result<(), validator::ValidationError> {
    if !resource.contains(':') {
        return Err(validator::ValidationError::new("resource_format")
            .with_message("resource must contain ':' separator (e.g. SERVICE_KEY:ENTITY_KEY)".into()));
    }
    let parts: Vec<&str> = resource.splitn(2, ':').collect();
    if parts[0].is_empty() || parts[1].is_empty() {
        return Err(validator::ValidationError::new("resource_format")
            .with_message("resource must have non-empty service key and entity key".into()));
    }
    let valid_chars = |s: &str| s.chars().all(|c| c.is_ascii_uppercase() || c == '_' || c.is_ascii_digit());
    if !valid_chars(parts[0]) || !valid_chars(parts[1]) {
        return Err(validator::ValidationError::new("resource_format")
            .with_message("resource must use UPPER_SNAKE_CASE (e.g. AUTH:ROLE)".into()));
    }
    Ok(())
}

#[derive(Deserialize, Serialize, Validate, Debug, ToSchema)]
pub struct PermissionForCreateRequest {
    #[validate(
        length(min = 5, max = 1024, code = "resource_length", message = "the length of resource must be between 5 and 1024"),
        custom(function = "validate_resource_format")
    )]
    pub resource: String,
    pub description: Option<String>,
    pub mask: Option<i32>,
}

impl Into<PermissionForCreateDto> for PermissionForCreateRequest {
    fn into(self) -> PermissionForCreateDto {
        PermissionForCreateDto {
            resource: self.resource,
            description: self.description,
            mask: self.mask.unwrap_or(0), // Default mask value
        }
    }
}

#[derive(Deserialize, Serialize, Validate, Debug, ToSchema)]
pub struct PermissionForUpdateRequest {
    #[validate(
        length(min = 5, max = 1024, code = "resource_length", message = "the length of resource must be between 5 and 1024"),
        custom(function = "validate_resource_format")
    )]
    pub resource: String,
    pub description: Option<String>,
    pub mask: Option<i32>,
}

impl Into<PermissionForCreateRequestDto> for PermissionForUpdateRequest {
    fn into(self) -> PermissionForCreateRequestDto {
        PermissionForCreateRequestDto {
            resource: self.resource,
            description: self.description,
            mask: self.mask.unwrap_or(0), // Default mask value
        }
    }
}

use shared_shared_data_core::{
    filter::{FilterEnum, FilterParam},
    filter_deserialize::*,
};
#[derive(Serialize, Deserialize, Debug, ToSchema, Default, Response, ParamFilter)]
pub struct PermissionData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mask: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Uuid>,
}

impl PermissionData {
    /// Return a new PermissionData with only the selected fields populated.
    pub fn filter_fields(mut self, fields: &Vec<String>) -> Self {
        if !fields.contains(&"id".to_string()) {
            self.id = None;
        }
        if !fields.contains(&"resource".to_string()) {
            self.resource = None;
        }
        if !fields.contains(&"description".to_string()) {
            self.description = None;
        }
        if !fields.contains(&"mask".to_string()) {
            self.mask = None;
        }
        self
    }
}

impl Into<PermissionData> for ModelOptionDto {
    fn into(self) -> PermissionData {
        PermissionData {
            resource: self.resource,
            description: self.description.unwrap(),
            id: self.id,
            mask: self.mask,
            ..Default::default()
        }
    }
}

impl Into<PermissionData> for Model {
    fn into(self) -> PermissionData {
        PermissionData {
            resource: Some(self.resource),
            description: self.description,
            id: Some(self.id),
            mask: Some(self.mask),
            ..Default::default()
        }
    }
}
