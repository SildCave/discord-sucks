use crate::cloudflare::TurnstileResult;
use axum::http::StatusCode;


pub async fn register_user(
    
) -> (StatusCode, String) {
    // if turnstile_result == TurnstileResult::Allowed {
    //     return (StatusCode::OK, "User registered".to_string());
    // }

    return (StatusCode::UNAUTHORIZED, "Unauthorized".to_string());

}
