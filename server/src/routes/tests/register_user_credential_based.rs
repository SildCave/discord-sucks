#[cfg(test)]
pub(super) mod tests {
    use axum::{
        body::Body,
        http::{
            self, response, Method, Request
        }, Router
    };
    use pretty_assertions::assert_eq;
    use reqwest::blocking::multipart::Form;
    use serial_test::serial;
    use tower::util::ServiceExt;
    use tower_http::trace::{DefaultMakeSpan, TraceLayer};
    use axum::body::to_bytes;
    use crate::{
        app_objects::User,
        credentials::{
            Password,
            SaltMode
        },
        logs,
        routes::tests::{authenticate::tests::get_authenticate_endpoint_response_and_status_code, preparation::{
            get_axum_app,
            get_config,
            get_db_client
        }}
    };



    #[tokio::test]
    #[serial]
    async fn test_register_user_credential_based_success() {
        //logs::setup_logging().unwrap();
        let mut config = get_config();
        config.cloudflare.allow_invalid_turnstile = true;
        let app = get_axum_app(Some(config)).await;
        let trace_layer = TraceLayer::new_for_http()
            .make_span_with(DefaultMakeSpan::default().include_headers(true));
        let app = app.layer(
            trace_layer.clone()
        );
        //logs::setup_logging().unwrap();

        let body = format!(
            r#"email=niadg%40sjda.sd&username=sadas&password=Test123!x112d&date_of_birth=2024-10-27&cf-turnstile-response=1222"#
        );



        let req = Request::builder()
            .method(Method::POST)
            .uri("/register_user")
            .header("content-type", "application/x-www-form-urlencoded")
            .body(body)
            .unwrap();

        let response = app
            .oneshot(req)
            .await
            .unwrap();
        let status_code = response.status();

        let body_bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let body_str = String::from_utf8(body_bytes.to_vec())
            .expect("Failed to convert body to string");

        assert_eq!("User registered", body_str);
        assert_eq!(status_code.as_u16(), 200);
    }

    #[tokio::test]
    #[serial]
    async fn test_register_user_credential_based_bad_password() {
        //logs::setup_logging().unwrap();
        let mut config = get_config();
        config.cloudflare.allow_invalid_turnstile = true;
        let app = get_axum_app(Some(config)).await;
        let trace_layer = TraceLayer::new_for_http()
            .make_span_with(DefaultMakeSpan::default().include_headers(true));
        let app = app.layer(
            trace_layer.clone()
        );
        //logs::setup_logging().unwrap();

        let body = format!(
            r#"email=niadg%40sjda.sd&username=sadas&password=aaax112d&date_of_birth=2024-10-27&cf-turnstile-response=1222"#
        );



        let req = Request::builder()
            .method(Method::POST)
            .uri("/register_user")
            .header("content-type", "application/x-www-form-urlencoded")
            .body(body)
            .unwrap();

        let response = app
            .oneshot(req)
            .await
            .unwrap();
        let status_code = response.status();

        let body_bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let body_str = String::from_utf8(body_bytes.to_vec())
            .expect("Failed to convert body to string");

        assert_eq!("1100", body_str);
        assert_eq!(status_code.as_u16(), 400);
    }

    
    #[tokio::test]
    #[serial]
    async fn test_register_user_credential_based_bad_email() {
        //logs::setup_logging().unwrap();
        let mut config = get_config();
        config.cloudflare.allow_invalid_turnstile = true;
        let app = get_axum_app(Some(config)).await;
        let trace_layer = TraceLayer::new_for_http()
            .make_span_with(DefaultMakeSpan::default().include_headers(true));
        let app = app.layer(
            trace_layer.clone()
        );
        //logs::setup_logging().unwrap();

        let body = format!(
            r#"email=niadgsjda.com&username=sadas&password=Test123!x112d&date_of_birth=2024-10-27&cf-turnstile-response=1222"#
        );



        let req = Request::builder()
            .method(Method::POST)
            .uri("/register_user")
            .header("content-type", "application/x-www-form-urlencoded")
            .body(body)
            .unwrap();

        let response = app
            .oneshot(req)
            .await
            .unwrap();
        let status_code = response.status();

        let body_bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let body_str = String::from_utf8(body_bytes.to_vec())
            .expect("Failed to convert body to string");

        assert_eq!("Invalid email", body_str);
        assert_eq!(status_code.as_u16(), 400);
    }

}