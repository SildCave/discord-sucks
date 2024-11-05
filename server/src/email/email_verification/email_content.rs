use lettre::{message::Mailbox, Message};

use crate::email::{EmailAuthor, EmailHandlerError};


pub fn create_email_verification_email(
    email_author: Mailbox,
    recipient: Mailbox
) -> Result<Message, EmailHandlerError> {
    let message = Message::builder()
        .from(email_author)
        .to(recipient);
    unimplemented!()
}

