#[cfg(test)]
pub(super) mod tests {
    use axum::{
        body::Body,
        http::{
            response, Method, Request
        }, Router
    };
    use pretty_assertions::assert_eq;
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

    pub async fn get_refresh_token_from_authenticate_endpoint(
        app: Router
    ) -> String {
        let db_client = get_db_client().await;
        db_client.redis_delete_password_hash_by_user_id(420).await.unwrap();
        let config = get_config();
        let test_password = Password::new(
            "test_password123*&@#ABC",
            &config.password_requirements
        );
        let salt_string = "ExampleSaltStringExampleSaltString";
        let salt = SaltMode::FromString(salt_string);
        let hash = test_password.hash_and_salt_password(
            &salt
        ).await.unwrap().password_hash;
        let user = User {
            id: 420,
            username: "test_user".to_string(),
            email: Some("test_user@gmail.com".to_string()),
            password_hash: hash,
            salt: salt_string.to_string(),
            ..User::default()
        };
        let res = db_client.postgres_delete_user_by_id(420).await;
        if res.is_err() {
            match res.err().unwrap() {
                crate::database::DatabaseError::UserNotFound(_) => {},
                _ => panic!("Error deleting user")
            }
        }
        db_client.postgres_insert_user(&user).await.unwrap();

        let (response, status_code) = get_authenticate_endpoint_response_and_status_code(
            test_password.get_password(),
            user.email.as_ref().unwrap(),
            app
        ).await;
        println!("Response1: {}", response);

        

        assert_eq!(status_code, 200);
        let response: serde_json::Value = serde_json::from_str(&response).unwrap();
        return response["refresh_token"].as_str().unwrap().to_string();
    }

    #[tokio::test]
    #[serial]
    async fn test_refresh_token_endpoint() {
        let app = get_axum_app().await;
        let trace_layer = TraceLayer::new_for_http()
            .make_span_with(DefaultMakeSpan::default().include_headers(true));
        let app = app.layer(
            trace_layer.clone()
        );
        //logs::setup_logging().unwrap();
        let db_client = get_db_client().await;
        let _ = db_client.cached_delete_user_refresh_token(420).await;
        let refresh_token = get_refresh_token_from_authenticate_endpoint(
            app.clone()
        ).await;

        println!("Refresh token: {}", refresh_token);

        let cookie = format!("refresh_token={}", refresh_token);
        let req = Request::builder()
            .method(Method::POST)
            .uri("/refresh_token")
            .header("Content-Type", "application/json")
            .header("Cookie", cookie)
            .body(Body::empty())
            .unwrap();
        let response = app
            .oneshot(req)
            .await
            .unwrap();
        let status_code = response.status();

        let body_bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let body_str = String::from_utf8(body_bytes.to_vec())
            .expect("Failed to convert body to string");

        println!("Response: {}", body_str);
        assert_eq!(status_code.as_u16(), 200);

        db_client.postgres_delete_user_by_id(420).await.unwrap();
    }

    #[tokio::test]
    #[serial]
    async fn test_refresh_token_endpoint_no_token() {
        let app = get_axum_app().await;
        let trace_layer = TraceLayer::new_for_http()
            .make_span_with(DefaultMakeSpan::default().include_headers(true));
        let app = app.layer(
            trace_layer.clone()
        );
        //logs::setup_logging().unwrap();

        let req = Request::builder()
            .method(Method::POST)
            .uri("/refresh_token")
            .header("Content-Type", "application/json")
            .body(Body::empty())
            .unwrap();
        let response = app
            .oneshot(req)
            .await
            .unwrap();
        let status_code = response.status();

        let body_bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let body_str = String::from_utf8(body_bytes.to_vec())
            .expect("Failed to convert body to string");

        println!("Response: {}", body_str);
        let response: serde_json::Value = serde_json::from_str(&body_str).unwrap();
        assert_eq!(status_code.as_u16(), 400);
        assert_eq!(response["error"], "No token");

    }

    #[tokio::test]
    #[serial]
    async fn test_refresh_token_endpoint_invalid_token() {
        let app = get_axum_app().await;
        let trace_layer = TraceLayer::new_for_http()
            .make_span_with(DefaultMakeSpan::default().include_headers(true));
        let app = app.layer(
            trace_layer.clone()
        );
        //logs::setup_logging().unwrap();

        let cookie = "refresh_token=invalid_token";
        let req = Request::builder()
            .method(Method::POST)
            .uri("/refresh_token")
            .header("Content-Type", "application/json")
            .header("Cookie", cookie)
            .body(Body::empty())
            .unwrap();
        let response = app
            .oneshot(req)
            .await
            .unwrap();
        let status_code = response.status();

        let body_bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let body_str = String::from_utf8(body_bytes.to_vec())
            .expect("Failed to convert body to string");

        println!("Response: {}", body_str);
        let response: serde_json::Value = serde_json::from_str(&body_str).unwrap();
        assert_eq!(status_code.as_u16(), 400);
        assert_eq!(response["error"], "Invalid token");

    }
}