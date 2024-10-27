use serde::{Deserialize, Serialize};

use crate::auth::ClaimType;

#[derive(Debug, Serialize)]
pub struct AuthenticationBody {
    pub access_token: String,
    pub token_type: String,
}
impl AuthenticationBody {
    pub fn new(access_token: String) -> Self {
        Self {
            access_token,
            token_type: ClaimType::Refresh.as_str().to_string(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct AuthenticationPayload {
    pub email: String,
    pub password: String,
}
