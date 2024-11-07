use lettre::{message::Mailbox, Address};
use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct EmailVerificationEmailState {
    pub email_sender_name: String,
    pub email_sender_email_address: String,
    pub email_subject: String,
    pub verification_url_domain: String,
    pub verification_url_endpoint: String,
    pub email_verification_jwt_lifetime_s: i64,
}

impl EmailVerificationEmailState {
    fn get_verification_email_email_address(&self) -> Address {
        self.email_sender_email_address.clone().parse().unwrap()
    }
    pub fn get_verification_email_author_mailbox(&self) -> Mailbox {
        Mailbox::new(
            Some(self.email_sender_name.clone()),
            self.get_verification_email_email_address()
        )
    }
}