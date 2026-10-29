use sanitizer::prelude::Sanitizer;
use serde::{Deserialize, Serialize};
use validator::Validate;
use sanitizer::prelude::*;

#[derive(Debug, Serialize, Deserialize, Validate, Sanitizer)]
#[serde(rename_all = "camelCase")]
pub struct CreateAdminUser {
    #[sanitizer(trim, lower_case)]
    pub first_name: String,
    #[sanitizer(trim, lower_case)]
    pub last_name: String,
    #[sanitizer(trim, lower_case)]
    #[validate(email)]
    pub email: String,
    #[sanitizer(trim)]
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Validate, Sanitizer)]
#[serde(rename_all = "camelCase")]
pub struct LoginAdminUser {
    #[sanitizer(trim, lower_case)]
    #[validate(email)]
    pub email: String,
    #[sanitizer(trim)]
    pub password: String,
}

