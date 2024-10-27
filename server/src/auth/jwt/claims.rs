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
pub struct Claims {
    // Issued at
    pub iat: i64,
    // Token expiration in unix timestamp
    pub exp: i64,
    // Token type
    pub claim_type: ClaimType,
}

impl Claims {
    /// Check if claim type is equal to the provided claim type
    pub fn valid_type(&self, claim_type: ClaimType) -> bool {
        self.claim_type == claim_type
    }

    pub fn new_access(
        lifetime: i64
    ) -> Self {
        let now = chrono::Utc::now().timestamp();
        Self {
            iat: now,
            exp: (now + lifetime),
            claim_type: ClaimType::Access,
        }
    }

    pub fn new_refresh(
        lifetime: i64
    ) -> Self {
        let now = chrono::Utc::now().timestamp();
        Self {
            iat: now,
            exp: (now + lifetime),
            claim_type: ClaimType::Refresh,
        }
    }
}