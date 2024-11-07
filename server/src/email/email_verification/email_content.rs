use lettre::{message::Mailbox, Executor, Message};

use crate::email::{EmailHandler, EmailHandlerError};


const EMAIL_VERIFICATION_SUBJECT: &'static str = "Discord-Sucks email verification";

impl EmailHandler{
    pub fn create_email_verification_email(
        &self,
        recipient: Mailbox,
        jwt_encoded_user_data: String
    ) -> Result<Message, EmailHandlerError> {
        let email_author = self.state.verification_email_state.get_verification_email_author_mailbox();
        let body = format!(
            "{}?token={}",
            "http://localhost:3000/email-verification",
            jwt_encoded_user_data
        );
        let message = Message::builder()
            .from(email_author)
            .to(recipient)
            .subject(EMAIL_VERIFICATION_SUBJECT)
            .body(body)
            .map_err(
                |e| EmailHandlerError::EmailCreationFailed(
                    e.to_string()
                )
            )?;
        
        Ok(message)
    }
}


