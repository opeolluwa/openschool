use sanitizer::prelude::Sanitizer;
use sanitizer::prelude::*;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, Validate, Sanitizer)]
#[serde(rename_all = "camelCase")]
pub struct ContactAddressPayload {
    #[sanitizer(trim)]
    pub address_line_one: String,
    #[sanitizer(trim)]
    pub address_line_two: Option<String>,
    #[sanitizer(trim)]
    pub city: String,
    #[sanitizer(trim)]
    pub region: Option<String>,
    #[sanitizer(trim)]
    pub country: String,
    #[sanitizer(trim)]
    pub zip_code: String,
}
