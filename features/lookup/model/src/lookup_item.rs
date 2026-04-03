use chrono::NaiveDateTime as DateTime;
use serde::{Deserialize, Serialize};
use serde_json::Value as Json;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

use shared_shared_data_core::{
    filter::{FilterEnum, FilterParam, VecString},
    filter_deserialize::*,
};
use shared_shared_macro::{ParamFilter, Response};

use features_lookup_entities::lookup_item::{
    LookupItemForCreateDto, LookupItemForUpdateDto, ModelOptionDto,
};

use super::lookup_item_translation::LookupItemTranslationData;

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema, Default, Response, ParamFilter)]
pub struct LookupItemData {
    pub id: Option<Uuid>,
    pub lookup_type_id: Option<Uuid>,
    pub code: Option<String>,
    pub name: Option<String>,
    pub url: Option<String>,
    pub query_param_one: Option<String>,
    pub query_param_two: Option<String>,
    pub tenants: Option<VecString>,
    pub is_active: Option<bool>,
    pub sort_order: Option<i32>,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
}

impl Into<LookupItemData> for ModelOptionDto {
    fn into(self) -> LookupItemData {
        LookupItemData {
            id: self.id,
            lookup_type_id: self.lookup_type_id,
            code: self.code,
            name: self.name,
            url: self.url,
            query_param_one: self.query_param_one,
            query_param_two: self.query_param_two,
            tenants: self.tenants,
            is_active: self.is_active,
            sort_order: self.sort_order,
            created_at: self.created_at,
            updated_at: self.updated_at,
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct LookupItemWithTranslations {
    pub item: LookupItemData,
    pub translations: Vec<LookupItemTranslationData>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct LookupItemForCreateRequest {
    pub lookup_type_id: Uuid,
    #[validate(length(
        min = 1,
        max = 50,
        code = "lookup_item_code_length",
        message = "code must be between 1 and 50 characters"
    ))]
    pub code: String,
    #[validate(length(
        min = 1,
        max = 200,
        code = "lookup_item_name_length",
        message = "name must be between 1 and 200 characters"
    ))]
    pub name: String,
    #[validate(length(
        max = 500,
        code = "lookup_item_url_length",
        message = "url must not exceed 500 characters"
    ))]
    pub url: Option<String>,
    #[validate(length(
        max = 200,
        code = "lookup_item_query_param_one_length",
        message = "query_param_one must not exceed 200 characters"
    ))]
    pub query_param_one: Option<String>,
    #[validate(length(
        max = 200,
        code = "lookup_item_query_param_two_length",
        message = "query_param_two must not exceed 200 characters"
    ))]
    pub query_param_two: Option<String>,
    pub tenants: Option<VecString>,
    pub sort_order: Option<i32>,
}

impl Into<LookupItemForCreateDto> for LookupItemForCreateRequest {
    fn into(self) -> LookupItemForCreateDto {
        LookupItemForCreateDto {
            lookup_type_id: self.lookup_type_id,
            code: self.code,
            name: self.name,
            url: self.url.unwrap_or_default(),
            query_param_one: self.query_param_one.unwrap_or_default(),
            query_param_two: self.query_param_two.unwrap_or_default(),
            tenants: self.tenants.unwrap_or_default(),
            is_active: true,
            sort_order: self.sort_order.unwrap_or(0),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct LookupItemForUpdateRequest {
    #[validate(length(
        min = 1,
        max = 50,
        code = "lookup_item_code_length",
        message = "code must be between 1 and 50 characters"
    ))]
    pub code: Option<String>,
    #[validate(length(
        min = 1,
        max = 200,
        code = "lookup_item_name_length",
        message = "name must be between 1 and 200 characters"
    ))]
    pub name: Option<String>,
    #[validate(length(
        max = 500,
        code = "lookup_item_url_length",
        message = "url must not exceed 500 characters"
    ))]
    pub url: Option<String>,
    #[validate(length(
        max = 200,
        code = "lookup_item_query_param_one_length",
        message = "query_param_one must not exceed 200 characters"
    ))]
    pub query_param_one: Option<String>,
    #[validate(length(
        max = 200,
        code = "lookup_item_query_param_two_length",
        message = "query_param_two must not exceed 200 characters"
    ))]
    pub query_param_two: Option<String>,
    pub tenants: Option<VecString>,
    pub is_active: Option<bool>,
    pub sort_order: Option<i32>,
}

impl Into<LookupItemForUpdateDto> for LookupItemForUpdateRequest {
    fn into(self) -> LookupItemForUpdateDto {
        LookupItemForUpdateDto {
            code: self.code,
            name: self.name,
            url: self.url,
            query_param_one: self.query_param_one,
            query_param_two: self.query_param_two,
            tenants: self.tenants,
            is_active: self.is_active,
            sort_order: self.sort_order,
        }
    }
}
