use secrecy::SecretString;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PaymentBackend {
    Paystack,
}

impl std::fmt::Display for PaymentBackend {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Paystack => write!(f, "paystack"),
        }
    }
}

#[derive(Clone)]
pub struct RemittanceConfig {
    pub backend: PaymentBackend,
    pub paystack_api_key: SecretString,
    pub paystack_api_secret: SecretString,
    pub paystack_base_url: String,
    pub paystack_verify_base_url: String,
}
