use sea_orm::DatabaseConnection;

use crate::repositories::academic_calendar::AcademicCalendarRepository;
use crate::repositories::admin_user::AdminUserRepository;
use crate::repositories::base::Repository;
use crate::repositories::contact_address::ContactAddressRepository;
use crate::repositories::grading_system::GradingSystemRepository;
use crate::repositories::institution::InstitutionRepository;
use crate::repositories::otp_repository::OtpRepository;

#[derive(Clone)]
pub struct Repositories {
    pub otp: OtpRepository,
    pub contact_address: ContactAddressRepository,
    pub admin_user: AdminUserRepository,
    pub institution: InstitutionRepository,
    pub academic_calendar: AcademicCalendarRepository,
    pub grading_system: GradingSystemRepository,
}

impl Repositories {
    pub fn new(db_pool: &DatabaseConnection) -> Self {
        Self {
            otp: OtpRepository::init(db_pool),
            contact_address: ContactAddressRepository::init(db_pool),
            admin_user: AdminUserRepository::init(db_pool),
            institution: InstitutionRepository::init(db_pool),
            academic_calendar: AcademicCalendarRepository::init(db_pool),
            grading_system: GradingSystemRepository::init(db_pool),
        }
    }
}
