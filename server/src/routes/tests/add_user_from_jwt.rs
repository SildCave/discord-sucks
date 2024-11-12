#[cfg(test)]
pub(super) mod tests {
    use axum::{
        body::Body,
        http::{
            self, response, Method, Request
        }, Router
    };
    use chrono::NaiveDate;
    use pretty_assertions::assert_eq;
    use reqwest::blocking::multipart::Form;
    use serial_test::serial;
    use tower::util::ServiceExt;
    use tower_http::trace::{DefaultMakeSpan, TraceLayer};
    use axum::body::to_bytes;
    use crate::{
        app_objects::User, auth::JWTKeys, credentials::{
            Password,
            SaltMode
        }, logs, registration::UserRegistrationFormJWT, routes::tests::{authenticate::tests::get_authenticate_endpoint_response_and_status_code, preparation::{
            get_axum_app,
            get_config,
            get_db_client
        }}
    };

    use urlencoding;

    #[tokio::test]
    #[serial]
    async fn test_add_user_from_jwt_success() {
        //logs::setup_logging().unwrap();
        let mut config = get_config();
        let jwt_keys = JWTKeys::new(&config).unwrap();
        config.cloudflare.allow_invalid_turnstile = true;
        let app = get_axum_app(Some(config)).await;
        let trace_layer = TraceLayer::new_for_http()
            .make_span_with(DefaultMakeSpan::default().include_headers(true));
        let app = app.layer(
            trace_layer.clone()
        );
        let user_registration_form_jwt = UserRegistrationFormJWT {
            email: "test_email1".to_string(),
            username: "test_username".to_string(),
            password_hash: "test_password_hash".to_string(),
            password_salt: "test_password_salt".to_string(),
            date_of_birth: NaiveDate::from_ymd(2024, 10, 27),
            exp: u32::MAX as i64,
        };

        let jwt = user_registration_form_jwt.into_jwt_token(
            &jwt_keys
        ).unwrap();

        let url = format!("/verify_email?token={}", jwt);
        todo!("consult database to get user id and check if user is in db");
        let request = Request::builder()
            .method(Method::GET)
            .uri(url)
            .body(Body::empty())
            .unwrap();
        
        let response = app
            .oneshot(request)
            .await
            .unwrap();
        let status_code = response.status();
        let body_bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let body_str = String::from_utf8(body_bytes.to_vec())
            .expect("Failed to convert body to string");

        println!("response: {}", body_str);

        assert_eq!("User added", body_str);
        assert_eq!(status_code.as_u16(), 200);


        //logs::setup_logging().unwrap();

    }

}