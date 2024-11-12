

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
        routes::tests::preparation::{
            get_axum_app,
            get_config,
            get_db_client
        }
    };

    pub async fn get_authenticate_endpoint_response_and_status_code(
        password: &str,
        email: &str,
        app: Router
    ) -> (String, u16) {
        let body_string = format!(
            r#"{{"email": "{}", "password": "{}"}}"#,
            email,
            password
        );
        let req = Request::builder()
            .method(Method::POST)
            .uri("/authenticate")
            .header("content-type", "application/json")
            .body(Body::from(
                body_string
            ))
            .unwrap();


        let response = app
            .oneshot(req)
            .await
            .unwrap();
        let status_code = response.status();

        let body_bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let body_str = String::from_utf8(body_bytes.to_vec())
            .expect("Failed to convert body to string");

        // let body: Value = serde_json::from_str(&body_str)
        //     .expect("Failed to parse JSON");

        (body_str, status_code.as_u16())
    }

    #[tokio::test]
    #[serial]
    async fn test_authenticate() {
        let db_client = get_db_client().await;
        db_client.redis_delete_password_hash_by_user_id(420).await.unwrap();

        let app = get_axum_app(None).await;
        let trace_layer = TraceLayer::new_for_http()
            .make_span_with(DefaultMakeSpan::default().include_headers(true));
        let app = app.layer(
            trace_layer.clone()
        );
        //logs::setup_logging().unwrap();

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
            email: "test_email".to_string(),
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
            &user.email,
            app
        ).await;
        println!("Response: {}", response);

        db_client.postgres_delete_user_by_id(420).await.unwrap();

        assert_eq!(status_code, 200);
    }


    #[tokio::test]
    #[serial]
    async fn test_authenticate_invalid_password() {
        let db_client = get_db_client().await;
        db_client.redis_delete_password_hash_by_user_id(420).await.unwrap();

        let app = get_axum_app(None).await;
        let trace_layer = TraceLayer::new_for_http()
            .make_span_with(DefaultMakeSpan::default().include_headers(true));
        let app = app.layer(
            trace_layer.clone()
        );
        //logs::setup_logging().unwrap();

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
            email: "test_email".to_string(),
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
            format!("{}INVALID", test_password.get_password()).as_str(),
            &user.email,
            app
        ).await;
        println!("Response: {}", response);

        db_client.postgres_delete_user_by_id(420).await.unwrap();

        let response: serde_json::Value = serde_json::from_str(&response).unwrap();
        assert_eq!(response["error"], "Wrong credentials");
        assert_eq!(status_code, 401);
    }

    #[tokio::test]
    #[serial]
    async fn test_authenticate_invalid_email() {
        let db_client = get_db_client().await;
        db_client.redis_delete_password_hash_by_user_id(420).await.unwrap();

        let app = get_axum_app(None).await;
        let trace_layer = TraceLayer::new_for_http()
            .make_span_with(DefaultMakeSpan::default().include_headers(true));
        let app = app.layer(
            trace_layer.clone()
        );
        //logs::setup_logging().unwrap();

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
            email: "test_email".to_string(),
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
            format!("{}INVALID", &user.email).as_str(),
            app
        ).await;
        println!("Response: {}", response);

        db_client.postgres_delete_user_by_id(420).await.unwrap();
        let response: serde_json::Value = serde_json::from_str(&response).unwrap();
        assert_eq!(response["error"], "Wrong credentials");

        assert_eq!(status_code, 401);
    }
}