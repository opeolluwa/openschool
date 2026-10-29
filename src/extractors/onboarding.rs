use sanitizer::prelude::Sanitizer;
use sanitizer::prelude::*;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::models::sea_orm_active_enums::{AcademicCalendarType, GradingSystemType};

#[derive(Debug, Serialize, Deserialize, Validate, Sanitizer)]
#[serde(rename_all = "camelCase")]
pub struct RegisterAcademics {
    #[sanitizer(trim)]
    pub calendar_name: String,
    pub academic_calendar_type: AcademicCalendarType,
    #[sanitizer(trim)]
    pub grading_system_name: String,
    pub grading_system_type: GradingSystemType,
}
