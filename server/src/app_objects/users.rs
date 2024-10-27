#[derive(Debug, PartialEq)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub password_hash: String,
    pub salt: String,
    pub email: Option<String>,
    pub created_at: i64,
    pub valid_refresh_token: Option<String>,
    pub verified: bool
}

