use serde::{Deserialize, Serialize};

use crate::auth::ClaimType;

#[derive(Debug, Serialize)]
pub struct AuthenticationBody {
    pub refresh_token: String,
    pub token_type: String,
}
impl AuthenticationBody {
    pub fn new(refresh_token: String) -> Self {
        Self {
            refresh_token,
            token_type: ClaimType::Refresh.as_str().to_string(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct AuthenticationPayload {
    pub email: String,
    pub password: String,
}

