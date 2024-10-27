use axum::http::StatusCode;

use crate::auth::{ClaimType, Claims};

pub async fn secured(claims: Claims) -> (StatusCode, String) {
    if !claims.valid_type(ClaimType::Access) {
        return (StatusCode::UNAUTHORIZED, "Unauthorized".to_string());
    }

    return (StatusCode::OK, "Secured".to_string());
}