use sea_orm::DatabaseConnection;

use crate::repositories::base::Repository;
use crate::repositories::contact_address::ContactAddressRepository;
use crate::repositories::otp_repository::OtpRepository;

#[derive(Clone)]
pub struct Repositories {
    pub otp: OtpRepository,
    pub contact_address: ContactAddressRepository,
}

impl Repositories {
    pub fn new(db_pool: &DatabaseConnection) -> Self {
        Self {
            otp: OtpRepository::init(db_pool),
            contact_address: ContactAddressRepository::init(db_pool),
        }
    }
}
