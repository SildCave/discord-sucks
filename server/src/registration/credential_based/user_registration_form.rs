use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct UserRegistrationForm {
    pub email: String,
    pub password: String,
    pub date_of_birth: NaiveDate,
}