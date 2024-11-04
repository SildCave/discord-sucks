use crate::cloudflare::TurnstileResult;
use axum::http::StatusCode;
use tracing::info;


pub async fn register_user(
    turnstile_result: TurnstileResult
) -> (StatusCode, String) {
    // if turnstile_result == TurnstileResult::Allowed {
    //     return (StatusCode::OK, "User registered".to_string());
    // }
    info!("turnstile_result: {:?}", turnstile_result);
    if turnstile_result == TurnstileResult::Denied {
        return (StatusCode::FORBIDDEN, "Forbidden".to_string());
    }
    return (StatusCode::OK, "User registered".to_string());

}
