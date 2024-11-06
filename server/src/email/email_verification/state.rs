use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct EmailVerificationEmailState {
    pub email_sender_name: String,
    pub email_sender_email_address: String,
    pub email_subject: String,
    pub verification_url_domain: String,
    pub verification_url_endpoint: String,
}