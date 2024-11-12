use serde::{
    Deserialize,
    Serialize
};


#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum ClaimType {
    Access,
    Refresh,
}

impl ClaimType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ClaimType::Access => "authorization_token",
            ClaimType::Refresh => "refresh_token"
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthClaims {
    // Issued at
    pub iat: i64,
    // Token expiration in unix timestamp
    pub exp: i64,
    // Token type
    pub claim_type: ClaimType,
    // User id
    pub user_id: i64,
}

impl AuthClaims {
    /// Check if claim type is equal to the provided claim type
    pub fn valid_type(&self, claim_type: ClaimType) -> bool {
        self.claim_type == claim_type
    }

    pub fn new_access(
        lifetime: i64,
        user_id: i64
    ) -> Self {
        let now = chrono::Utc::now().timestamp();
        Self {
            iat: now,
            exp: (now + lifetime),
            claim_type: ClaimType::Access,
            user_id,
        }
    }

    pub fn new_refresh(
        lifetime: i64,
        user_id: i64
    ) -> Self {
        let now = chrono::Utc::now().timestamp();
        Self {
            iat: now,
            exp: (now + lifetime),
            claim_type: ClaimType::Refresh,
            user_id,
        }
    }
}