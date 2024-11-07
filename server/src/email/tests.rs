

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;
    use lettre::message::Mailbox;
    use crate::email::EmailHandler;
    use crate::registration::UserRegistrationFormJWT;
    use crate::routes::tests::preparation;
    use std::thread::sleep;
    use std::time::Duration;

    use pretty_assertions::assert_eq;

    #[tokio::test]
    async fn test_email_handler_error() {
        let config = preparation::get_config();
        let email_handler = EmailHandler::new(&config).unwrap();

        let recipient: Mailbox = "bogolskibob56@gmail.com".parse().unwrap();

        let email = email_handler.create_email_verification_email(
            recipient,
            "test".to_string()
        ).unwrap();

        email_handler.send_email(email).await.unwrap();
    }

    #[tokio::test]
    async fn test_user_to_jwt_and_the_other_way_around() {
        let config = preparation::get_config();

        let registration_form = UserRegistrationFormJWT::new(
            "123@123.pl".to_string(),
            "123".to_string(),
            NaiveDate::from_ymd_opt(2021, 1, 1).unwrap(),
            10
        );

        let jwt_keys = crate::auth::JWTKeys::new(&config).unwrap();
        let jwt_token = registration_form.into_jwt_token(&jwt_keys).unwrap();
        let registration_form_from_jwt = UserRegistrationFormJWT::from_jwt_token(&jwt_token, &jwt_keys).unwrap();

        assert_eq!(registration_form, registration_form_from_jwt);
    }

    #[tokio::test]
    async fn test_user_to_jwt_and_the_other_way_around_with_expired_token() {
        let config = preparation::get_config();

        let registration_form = UserRegistrationFormJWT::new(
            "123@123.pl".to_string(),
            "123".to_string(),
            NaiveDate::from_ymd_opt(2021, 1, 1).unwrap(),
            1
        );
        let jwt_keys = crate::auth::JWTKeys::new(&config).unwrap();
        let jwt_token = registration_form.into_jwt_token(&jwt_keys).unwrap();


        sleep(Duration::from_secs_f32(2.0));

        let registration_form_from_jwt = UserRegistrationFormJWT::from_jwt_token(&jwt_token, &jwt_keys);

        assert_eq!(true, registration_form_from_jwt.is_err());
    }
}