use chrono::{
    NaiveDate
};

#[derive(Debug, PartialEq)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub password_hash: String,
    pub salt: String,
    pub email: String,
    pub created_at: i64,
    pub valid_refresh_token: Option<String>,
    pub verified: bool,
    pub banned: bool,
    pub date_of_birth: NaiveDate
}


impl Default for User {
    fn default() -> Self {
        User {
            id: 0,
            username: "".to_string(),
            password_hash: "".to_string(),
            salt: "".to_string(),
            email: "".to_string(),
            created_at: 0,
            valid_refresh_token: None,
            verified: true,
            banned: false,
            date_of_birth: NaiveDate::default()
        }
    }
    
}
