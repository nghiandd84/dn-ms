use serde::{Deserialize, Serialize};
use std::{borrow::Cow, collections::HashMap};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::{Validate, ValidationError};

use features_auth_entities::user::{ModelOptionDto, UserForCreateDto};
use shared_shared_data_auth::password::hash;
use shared_shared_macro::{ParamFilter, Response};

#[derive(Deserialize, Serialize, Validate, Debug, ToSchema)]
#[validate(schema(function = "validate_register_request"))]
pub struct UserForCreateRequest {
    #[validate(length(
        min = 6,
        max = 256,
        code = "email_length",
        message = "the length of email must be between 6 and 256"
    ))]
    pub email: String,
    #[validate(length(
        min = 10,
        max = 128,
        code = "password_length",
        message = "the length of email must be between 10 and 128"
    ))]
    pub password: String,
    #[validate(length(
        min = 4,
        max = 16,
        code = "firstname_length",
        message = "the length of first_name must be between 4 and 16"
    ))]
    pub first_name: String,
    pub last_name: String,
}

impl Into<UserForCreateDto> for UserForCreateRequest {
    fn into(self) -> UserForCreateDto {
        let password = hash(self.password).unwrap_or_default();
        UserForCreateDto {
            email: self.email,
            first_name: self.first_name,
            last_name: self.last_name,
            password: password,
        }
    }
}

fn validate_register_request(
    request: &UserForCreateRequest,
) -> std::result::Result<(), ValidationError> {
    if request.email == "email_exist" {
        return Err(ValidationError {
            code: Cow::from("email_exist"),
            message: Some(Cow::from("email_exist error message")),
            params: HashMap::new(),
        });
    }
    Ok(())
}

use shared_shared_data_core::{
    filter::{FilterEnum, FilterParam},
    filter_deserialize::*,
};

#[derive(Serialize, Debug, ToSchema, Default, Response, ParamFilter)]
pub struct UserData {
    pub id: Option<Uuid>,
    first_name: Option<String>,
    last_name: Option<String>,
    email: Option<String>,
    age: Option<u32>,
}

impl Into<UserData> for ModelOptionDto {
    fn into(self) -> UserData {
        UserData {
            email: self.email,
            first_name: self.first_name,
            id: self.id,
            last_name: self.last_name,
            ..Default::default()
        }
    }
}

// impl Into<ModelOptionDto> for UserData {
//     fn into(self) -> ModelOptionDto {
//         ModelOptionDto {
//             email: self.email,
//             first_name: self.first_name,
//             id: self.id,
//             last_name: self.last_name,
//             ..Default::default()
//         }
//     }
// }
