use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct ContactAddressPayload {
    pub address_line_one: String,
    pub address_line_two: Option<String>,
    pub city: String,
    pub region: Option<String>,
    pub country: String,
    pub zip_code: String,
}
