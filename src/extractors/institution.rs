use sanitizer::prelude::Sanitizer;
use sanitizer::prelude::*;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::models::sea_orm_active_enums::InstitutionType;

#[derive(Debug, Serialize, Deserialize, Validate, Sanitizer)]
#[serde(rename_all = "camelCase")]
pub struct RegisterInstitution {
    #[sanitizer(trim)]
    #[validate(length(min = 1, message = "institution name is required"))]
    pub name: String,
    #[sanitizer(trim)]
    pub tagline: String,
    #[sanitizer(trim)]
    #[validate(url)]
    pub logo_url: Option<String>,
    pub institution_type: InstitutionType,
}
