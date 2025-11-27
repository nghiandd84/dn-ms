use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

use shared_shared_data_core::{
    filter::{FilterEnum, FilterParam},
    filter_deserialize::*,
};
use shared_shared_macro::{ParamFilter, Response};

use features_auth_entities::access::ModelOptionDto;
#[derive(Serialize, Debug, ToSchema, Default, Response, ParamFilter)]
pub struct UserAccessData {
    pub key: String,
    pub role_name: String,
}

#[derive(Serialize, Debug, ToSchema, Default, Response, ParamFilter)]
pub struct AccessData {
    id: Option<Uuid>,
    user_id: Option<Uuid>,
    role_id: Option<Uuid>,
    key: Option<String>,
}

impl Into<AccessData> for ModelOptionDto {
    fn into(self) -> AccessData {
        AccessData {
            id: self.id,
            role_id: self.role_id,
            user_id: self.user_id,
            key: self.key,
            ..Default::default()
        }
    }
}
