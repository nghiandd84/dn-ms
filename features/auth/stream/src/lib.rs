use serde::{Deserialize, Serialize};

use crate::{signin::SignInMessage, signup::SignUpMessage};

pub mod signin;
pub mod signup;

pub const PRODUCER_KEY: &str = "auth";

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "auth_type", rename_all = "snake_case")]
pub enum AuthMessage {
    SignIn { message: SignInMessage },
    SignUp { message: SignUpMessage },
}
