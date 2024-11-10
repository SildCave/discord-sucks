

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
        }, refresh_token::tests::get_refresh_token_from_authenticate_endpoint}
    };

    async fn get_authorization_token_from_refresh_token_endpoint(
        app: Router,
    ) -> String {
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
        let headers = response.headers();
        let cookie = headers.get("set-cookie").unwrap().to_str().unwrap();
        assert_eq!(status_code, 200);
        let cookie = cookie.split(";").collect::<Vec<&str>>()[0];
        let authorization_token = cookie.replace("authorization_token=Bearer ", "");
        authorization_token.to_string()
    }

    #[tokio::test]
    #[serial]
    async fn test_secured_endpoint() {
        let app = get_axum_app().await;
        let trace_layer = TraceLayer::new_for_http()
            .make_span_with(DefaultMakeSpan::default().include_headers(true));
        let app = app.layer(
            trace_layer.clone()
        );
        //logs::setup_logging().unwrap();
        let db_client = get_db_client().await;
        let _ = db_client.cached_delete_user_refresh_token(420).await;
        let access_token = get_authorization_token_from_refresh_token_endpoint(
            app.clone()
        ).await;

        println!("Access token: {}", access_token);

        let cookie = format!("authorization_token={}", access_token);

        let req = Request::builder()
            .method(Method::GET)
            .uri("/secured")
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
    async fn test_secured_endpoint_no_cookie() {
        let app = get_axum_app().await;
        let trace_layer = TraceLayer::new_for_http()
            .make_span_with(DefaultMakeSpan::default().include_headers(true));
        let app = app.layer(
            trace_layer.clone()
        );
        //logs::setup_logging().unwrap();

        let req = Request::builder()
            .method(Method::GET)
            .uri("/secured")
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
        assert_eq!(response["error"], "No token");
        assert_eq!(status_code.as_u16(), 400);
    }

    #[tokio::test]
    #[serial]
    async fn test_secured_endpoint_invalid_cookie() {
        let app = get_axum_app().await;
        let trace_layer = TraceLayer::new_for_http()
            .make_span_with(DefaultMakeSpan::default().include_headers(true));
        let app = app.layer(
            trace_layer.clone()
        );
        //logs::setup_logging().unwrap();

        let cookie = "authorization_token=Bearer invalid_token";

        let req = Request::builder()
            .method(Method::GET)
            .uri("/secured")
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
        assert_eq!(response["error"], "Invalid token");
        assert_eq!(status_code.as_u16(), 400);
    }
}