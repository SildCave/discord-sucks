use crate::configuration::Config;
use anyhow::Result;
mod email_verification;

#[derive(Debug, Clone)]
pub enum EmailHandlerError {
    EmailConstructionError(String),
}

#[derive(Debug, Clone, Copy)]
pub struct EmailHandler {
    state: &'static EmailHandlerState,
    url: &'static str,
}

impl EmailHandler {
    pub fn new<T>(
        config: &Config,
        email_sender_email_address: T,
        sender_name: T
    ) -> Result<Self>
    where T: AsRef<str> {
        let username = &config.smtp.smtp_username;
        let password = {
            let path = &config.smtp.smtp_password_path;
            let password = std::fs::read_to_string(path)?;
            password
        };

        let url = &config.smtp.smtp_host;

        let state = EmailHandlerState::new(
            username,
            &password,
            &sender_name.as_ref().to_string(),
            &email_sender_email_address.as_ref().to_string()
        );
        let url = Box::leak(
            Into::<String>::into(
                url
            ).into_boxed_str()
        );

        Ok(Self {
            state: Box::leak(
                Box::new(state)
            ),
            url,
        })
    }

}

#[derive(Debug, Clone, Copy)]
pub struct EmailAuthor {
    email: &'static str,
    name: &'static str,
}

impl EmailAuthor {
    pub fn new<T>(
        email: T,
        name: T
    ) -> Self
    where T: AsRef<str> {
        let email = Box::leak(
            Into::<String>::into(
                email.as_ref()
            ).into_boxed_str()
        );
        let name = Box::leak(
            Into::<String>::into(
                name.as_ref()
            ).into_boxed_str()
        );

        Self {
            email,
            name,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct EmailHandlerState {
    smtp_username: &'static str,
    smtp_password: &'static str,
    author: &'static EmailAuthor,
}

impl EmailHandlerState {
    // statics are kinda useless here but at least Im having fun
    pub fn new<T>(
        username: T,
        password: T,
        email_sender_name: T,
        email_sender_email_address: T
    ) -> Self
    where T: AsRef<str> {
        let smtp_username = Box::leak(
            Into::<String>::into(
                username.as_ref()
            ).into_boxed_str()
        );
        let smtp_password = Box::leak(
            Into::<String>::into(
                password.as_ref()
            ).into_boxed_str()
        );

        let author = EmailAuthor::new(
            email_sender_email_address,
            email_sender_name
        );

        Self {
            smtp_password,
            smtp_username,
            author: Box::leak(
                Box::new(author)
            ),
        }
    }
}
