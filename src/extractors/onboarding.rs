use sanitizer::prelude::Sanitizer;
use sanitizer::prelude::*;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate, Sanitizer)]
#[serde(rename_all = "camelCase")]
pub struct RegisterAcademics {
    #[sanitizer(trim)]
    pub calendar_name: String,
    #[sanitizer(trim)]
    pub academic_calendar_type: String,
    #[sanitizer(trim)]
    pub grading_system_name: String,
    #[sanitizer(trim)]
    pub grading_system_type: String,
}
