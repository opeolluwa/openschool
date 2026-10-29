use sanitizer::prelude::Sanitizer;
use sanitizer::prelude::*;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate, Sanitizer)]
#[serde(rename_all = "camelCase")]
pub struct RegisterInstitution {
    #[sanitizer(trim)]
    pub name: String,
    #[sanitizer(trim)]
    pub tagline: String,
    #[sanitizer(trim)]
    pub logo_url: Option<String>,
    #[sanitizer(trim)]
    pub institution_type: String,
}
