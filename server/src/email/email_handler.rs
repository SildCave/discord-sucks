use crate::configuration::Config;

use super::{
    email_handler_state::EmailHandlerState, email_verification::EmailVerificationEmailState, EmailHandlerError
};

use lettre::{
    Address,
    Message, Transport
};

#[derive(Debug, Clone, Copy)]
pub struct EmailHandler {
    state: &'static EmailHandlerState,
    smtp_host: &'static str,
}

impl EmailHandler {
    pub fn new(
        config: &Config,
    ) -> anyhow::Result<Self> {
        let smtp_username = &config.smtp.smtp_username;
        let smtp_password = &{
            let path = &config.smtp.smtp_password_path;
            let password = std::fs::read_to_string(path)?;
            password
        };

        let smtp_host = &config.smtp.smtp_host;

        let verification_email_state = EmailVerificationEmailState {
            email_sender_name: config.verification_email.email_sender_name.clone(),
            email_sender_email_address: config.verification_email.email_sender_email_address.clone(),
            email_subject: config.verification_email.email_subject.clone(),
            verification_url_domain: config.verification_email.verification_url_domain.clone(),
            verification_url_endpoint: config.verification_email.verification_url_endpoint.clone(),
        };


        let state = EmailHandlerState::new(
            smtp_username,
            smtp_password,
            smtp_host,
            verification_email_state
        );
        let smtp_host = Box::leak(
            Into::<String>::into(
                smtp_host
            ).into_boxed_str()
        );

        Ok(Self {
            state: Box::leak(
                Box::new(state)
            ),
            smtp_host,
        })
    }

    pub fn send_email(
        &self,
        mail: &Message
    ) -> Result<(), EmailHandlerError> {

        self.state.mailer().send(mail)
            .map_err(
                |e| EmailHandlerError::EmailSendingFailed(e.to_string())
            )?;

        Ok(())
    }
}


