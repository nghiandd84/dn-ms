use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

use shared_shared_data_core::{
    filter::{FilterEnum, FilterParam},
    filter_deserialize::*,
};
use shared_shared_macro::{ParamFilter, Response};

use features_auth_entities::access::ModelOptionDto;

#[derive(Deserialize, Serialize, Validate, Debug, ToSchema)]
pub struct AssignRoleToUserRequest {
    pub role_ids: Vec<Uuid>,
    pub key: Option<String>,
}
#[derive(Serialize, Debug, ToSchema, Default, Response, ParamFilter)]
pub struct UserAccessData {
    pub key: String,
    pub role_name: String,
}

#[derive(Serialize, Debug, ToSchema, Default, Response, ParamFilter)]
pub struct AccessData {
    pub id: Option<Uuid>,
    pub role_id: Option<Uuid>,
    pub key: Option<String>,
}

impl Into<AccessData> for ModelOptionDto {
    fn into(self) -> AccessData {
        AccessData {
            id: self.id,
            role_id: self.role_id,
            key: self.key,
            ..Default::default()
        }
    }
}

impl Into<AccessData> for features_auth_entities::access::Model {
    fn into(self) -> AccessData {
        AccessData {
            id: Some(self.id),
            role_id: Some(self.role_id),
            key: Some(self.key),
        }
    }
}
