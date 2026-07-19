use serde::{Deserialize, Serialize};
use shared_shared_macro::{ParamFilter, Response};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

use features_auth_entities::field_permission::{
    FieldPermissionForCreateDto, FieldPermissionForUpdateDto, Model, ModelOptionDto,
};

use shared_shared_data_core::{
    filter::{FilterEnum, FilterParam},
    filter_deserialize::*,
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
    let valid_chars =
        |s: &str| s.chars().all(|c| c.is_ascii_uppercase() || c == '_' || c.is_ascii_digit());
    if !valid_chars(parts[0]) || !valid_chars(parts[1]) {
        return Err(validator::ValidationError::new("resource_format")
            .with_message("resource must use UPPER_SNAKE_CASE (e.g. AUTH:ROLE)".into()));
    }
    Ok(())
}

#[derive(Deserialize, Serialize, Validate, Debug, ToSchema)]
pub struct FieldPermissionForCreateRequest {
    #[validate(required(code = "role_id_required", message = "role_id is required"))]
    pub role_id: Option<Uuid>,
    #[validate(
        required(code = "resource_required", message = "resource is required"),
        length(
            min = 5,
            max = 255,
            code = "resource_length",
            message = "resource must be between 5 and 255 characters"
        ),
        custom(function = "validate_resource_format")
    )]
    pub resource: Option<String>,
    #[validate(required(code = "action_required", message = "action is required"))]
    pub action: Option<i32>,
    #[validate(required(code = "fields_required", message = "fields is required"))]
    pub fields: Option<Vec<String>>,
}

impl Into<FieldPermissionForCreateDto> for FieldPermissionForCreateRequest {
    fn into(self) -> FieldPermissionForCreateDto {
        FieldPermissionForCreateDto {
            role_id: self.role_id.unwrap(),
            resource: self.resource.unwrap(),
            action: self.action.unwrap(),
            fields: self.fields.unwrap(),
        }
    }
}

#[derive(Deserialize, Serialize, Validate, Debug, ToSchema)]
pub struct FieldPermissionForUpdateRequest {
    pub fields: Option<Vec<String>>,
}

impl Into<FieldPermissionForUpdateDto> for FieldPermissionForUpdateRequest {
    fn into(self) -> FieldPermissionForUpdateDto {
        FieldPermissionForUpdateDto { fields: self.fields }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema, Default, Response, ParamFilter)]
pub struct FieldPermissionData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[skip_param]
    pub fields: Option<Vec<String>>,
}

impl From<ModelOptionDto> for FieldPermissionData {
    fn from(val: ModelOptionDto) -> Self {
        FieldPermissionData {
            id: val.id,
            role_id: val.role_id,
            resource: val.resource,
            action: val.action,
            fields: val.fields,
            ..Default::default()
        }
    }
}

impl From<Model> for FieldPermissionData {
    fn from(val: Model) -> Self {
        FieldPermissionData {
            id: Some(val.id),
            role_id: Some(val.role_id),
            resource: Some(val.resource),
            action: Some(val.action),
            fields: Some(val.fields),
            ..Default::default()
        }
    }
}
