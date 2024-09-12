use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct AuthenticationBody {
    pub access_token: String,
    pub token_type: String,
}
impl AuthenticationBody {
    pub fn new(access_token: String) -> Self {
        Self {
            access_token,
            token_type: "Bearer".to_string(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct AuthenticationPayload {
    pub client_id: String,
    pub client_secret: String,
}