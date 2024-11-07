use lettre::{message::Mailbox, transport::smtp::authentication::Credentials, Address, AsyncSmtpTransport, Executor, SmtpTransport, Tokio1Executor};

use super::email_verification::EmailVerificationEmailState;

#[derive(Debug, Clone, Copy)]
pub struct EmailHandlerState {
    mailer: &'static AsyncSmtpTransport<Tokio1Executor>,
    pub verification_email_state: &'static EmailVerificationEmailState,
}

impl EmailHandlerState {
    // statics are kinda useless here but at least Im having fun
    pub fn new<T>(
        smtp_username: T,
        smtp_password: T,
        smtp_host: T,
        verification_email_state: EmailVerificationEmailState

    ) -> EmailHandlerState
    where
        T: AsRef<str> {

        let smtp_credentials = Credentials::new(
            smtp_username.as_ref().to_string(),
            smtp_password.as_ref().to_string()
        );
        let mailer = AsyncSmtpTransport::<Tokio1Executor>::relay(
            smtp_host.as_ref()
        ).unwrap().credentials(
            smtp_credentials
        ).build();

        Self {
            mailer: Box::leak(
                Box::new(mailer)
            ),
            verification_email_state: Box::leak(
                Box::new(verification_email_state)
            ),
        }
    }

    pub fn mailer(&self) -> &'static AsyncSmtpTransport<Tokio1Executor> {
        self.mailer
    }
}
