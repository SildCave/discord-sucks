use std::ascii;

use serde::{Deserialize, Serialize};
use sqlx::any;
use thiserror::Error;

use super::password_preparation::SaltMode;


#[derive(Error, Debug, PartialEq)]
pub enum PasswordError {
    #[error("Password must be at least {0} characters long")]
    PasswordTooShort(usize),
    #[error("Password must contain at least {0} uppercase character")]
    PasswordNoUppercase(usize),
    #[error("Password must contain at least {0} symbol")]
    PasswordNoSymbol(usize),
    #[error("Password must contain at least {0} number")]
    PasswordNoNumber(usize),
    #[error("Password must contain only ASCII characters")]
    PasswordNotAscii,
    #[error("Password must not contain any special characters")]
    PasswordContainsSpecialCharacters,
    #[error("Password must not contain any whitespaces")]
    PasswordContainsWhitespaces,
    #[error("Password is too long, maximum length is {0}")]
    PasswordTooLong(usize),
    #[error("Password does not match hash")]
    PasswordDoesNotMatchHash,
    // Generic error
    #[error("Password preparation error: {0}")]
    HashError(String),
}


pub struct Password<'a> {
    password: &'a str,
    requirements: &'a PasswordRequirements,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PasswordRequirements {
    #[serde(rename = "min_length")]
    pub expected_min_length: usize,
    #[serde(rename = "max_length")]
    pub expected_max_length: usize,
    pub must_contain_uppercase: bool,
    pub must_contain_symbol: bool,
    pub must_contain_number: bool,
    pub ascii_only: bool,
    pub no_special_characters: bool,
    pub no_whitespaces: bool,
}

impl Default for PasswordRequirements {
    fn default() -> Self {
        Self {
            expected_min_length: 8,
            expected_max_length: 64,
            must_contain_uppercase: true,
            must_contain_symbol: true,
            must_contain_number: true,
            ascii_only: true,
            no_special_characters: true,
            no_whitespaces: true,
        }
    }
}

impl <'b>Password<'b> {
    pub fn new(
        password: &'b str,
        requirements: &'b PasswordRequirements
    ) -> Password<'b> {
        Self {
            password,
            requirements
        }
    }

    pub async fn check_if_password_matches_hash(
        &self,
        salt: &str,
        hash_to_compare: &str
    ) -> Result<bool, PasswordError> {
        let new_hash = self.hash_and_salt_password(
            &SaltMode::FromString(salt)
        ).await.map_err(
            |e| PasswordError::HashError(e.to_string())
        )?;
        if new_hash.password_hash != hash_to_compare {
            return Ok(false);
        } else {
            return Ok(true);
        }
    }

    pub fn get_password(&self) -> &str {
        &self.password
    }

    fn check_if_password_is_valid_based_on_requirements(&self) -> Result<(), PasswordError> {
        // Order of checks is important !!!
        if self.requirements.ascii_only {
            self.check_if_password_is_ascii()?;
        }
        if self.requirements.no_special_characters {
            self.check_if_special_characters_present()?;
        }
        self.check_length()?;
        if self.requirements.no_whitespaces {
            self.check_if_whitespace_present()?;
        }
        if self.requirements.must_contain_uppercase {
            self.check_if_uppercase_present()?;
        }
        if self.requirements.must_contain_symbol {
            self.check_if_symbol_present()?;
        }
        if self.requirements.must_contain_number {
            self.check_if_number_present()?;
        }
        Ok(())
    }

    fn check_length(&self) -> Result<(), PasswordError> {
        if self.password.len() < self.requirements.expected_min_length {
            return Err(
                PasswordError::PasswordTooShort(self.requirements.expected_min_length)
            );
        }
        if self.password.len() > self.requirements.expected_max_length {
            return Err(
                PasswordError::PasswordTooLong(self.requirements.expected_max_length)
            );
        }
        Ok(())
    }

    fn check_if_uppercase_present(&self) -> Result<(), PasswordError> {
        if !self.password.chars().any(|c| c.is_uppercase()) {
            return Err(PasswordError::PasswordNoUppercase(1));
        }
        Ok(())
    }

    fn check_if_password_is_ascii(&self) -> Result<(), PasswordError> {
        if !self.password.chars().all(|c| c.is_ascii()) {
            return Err(PasswordError::PasswordNotAscii);
        }
        Ok(())
    }

    fn check_if_symbol_present(&self) -> Result<(), PasswordError> {
        if !self.password.chars().any(|c| !c.is_alphanumeric()) {
            return Err(PasswordError::PasswordNoSymbol(1));
        }
        Ok(())
    }

    fn check_if_number_present(&self) -> Result<(), PasswordError> {
        if !self.password.chars().any(|c| c.is_numeric()) {
            return Err(PasswordError::PasswordNoNumber(1));
        }
        Ok(())
    }

    fn check_if_whitespace_present(&self) -> Result<(), PasswordError> {
        if self.password.chars().any(|c| c.is_whitespace()) {
            return Err(PasswordError::PasswordContainsWhitespaces);
        }
        Ok(())
    }

    fn check_if_special_characters_present(&self) -> Result<(), PasswordError> {
        if self.password.chars().any(|c| !matches!(c, ' '..='~')) {
            return Err(PasswordError::PasswordContainsSpecialCharacters);
        }
        Ok(())
    }


}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_successful_check_if_password_is_valid_based_on_requirements() {
        let requirements = PasswordRequirements {
            expected_min_length: 8,
            must_contain_uppercase: true,
            must_contain_symbol: true,
            must_contain_number: true,
            ascii_only: true,
            no_special_characters: true,
            no_whitespaces: true,
            expected_max_length: 64
        };
        let password = Password::new(
            "Password123!",
            &requirements
        );
        assert!(password.check_if_password_is_valid_based_on_requirements().is_ok());
    }

    #[test]
    fn test_password_too_short() {
        let requirements = PasswordRequirements {
            expected_min_length: 10,
            must_contain_uppercase: true,
            must_contain_symbol: true,
            must_contain_number: true,
            ascii_only: true,
            no_special_characters: true,
            no_whitespaces: true,
            expected_max_length: 64
        };
        let password = Password::new(
            "Pass123!",
            &requirements
        );
        assert!(password.check_if_password_is_valid_based_on_requirements().is_err());
        assert_eq!(
            password.check_if_password_is_valid_based_on_requirements().unwrap_err(),
            PasswordError::PasswordTooShort(10)
        );
    }

    #[test]
    fn test_password_too_long() {
        let requirements = PasswordRequirements {
            expected_min_length: 8,
            must_contain_uppercase: true,
            must_contain_symbol: true,
            must_contain_number: true,
            ascii_only: true,
            no_special_characters: true,
            no_whitespaces: true,
            expected_max_length: 10
        };
        let password = Password::new(
            "Password123!lol",
            &requirements
        );
        assert!(password.check_if_password_is_valid_based_on_requirements().is_err());
        assert_eq!(
            password.check_if_password_is_valid_based_on_requirements().unwrap_err(),
            PasswordError::PasswordTooLong(10)
        );
    }

    #[test]
    fn test_password_no_uppercase() {
        let requirements = PasswordRequirements {
            expected_min_length: 8,
            must_contain_uppercase: true,
            must_contain_symbol: true,
            must_contain_number: true,
            ascii_only: true,
            no_special_characters: true,
            no_whitespaces: true,
            expected_max_length: 64
        };
        let password = Password::new(
            "password123!",
            &requirements
        );
        assert!(password.check_if_password_is_valid_based_on_requirements().is_err());
        assert_eq!(
            password.check_if_password_is_valid_based_on_requirements().unwrap_err(),
            PasswordError::PasswordNoUppercase(1)
        );
    }

    #[test]
    fn test_password_no_symbol() {
        let requirements = PasswordRequirements {
            expected_min_length: 8,
            must_contain_uppercase: true,
            must_contain_symbol: true,
            must_contain_number: true,
            ascii_only: true,
            no_special_characters: true,
            no_whitespaces: true,
            expected_max_length: 64
        };
        let password = Password::new(
            "Password123",
            &requirements
        );
        assert!(password.check_if_password_is_valid_based_on_requirements().is_err());
        assert_eq!(
            password.check_if_password_is_valid_based_on_requirements().unwrap_err(),
            PasswordError::PasswordNoSymbol(1)
        );
    }

    #[test]
    fn test_password_no_number() {
        let requirements = PasswordRequirements {
            expected_min_length: 8,
            must_contain_uppercase: true,
            must_contain_symbol: true,
            must_contain_number: true,
            ascii_only: true,
            no_special_characters: true,
            no_whitespaces: true,
            expected_max_length: 64
        };
        let password = Password::new(
            "Password!",
            &requirements
        );
        assert!(password.check_if_password_is_valid_based_on_requirements().is_err());
        assert_eq!(
            password.check_if_password_is_valid_based_on_requirements().unwrap_err(),
            PasswordError::PasswordNoNumber(1)
        );
    }

    #[test]
    fn test_password_not_ascii() {
        let requirements = PasswordRequirements {
            expected_min_length: 8,
            must_contain_uppercase: true,
            must_contain_symbol: true,
            must_contain_number: true,
            ascii_only: true,
            no_special_characters: true,
            no_whitespaces: true,
            expected_max_length: 64
        };
        let password = Password::new(
            "Password123!🤣",
            &requirements
        );
        assert!(password.check_if_password_is_valid_based_on_requirements().is_err());
        assert_eq!(
            password.check_if_password_is_valid_based_on_requirements().unwrap_err(),
            PasswordError::PasswordNotAscii
        );
    }

    #[test]
    fn test_password_contains_special_characters() {
        let requirements = PasswordRequirements {
            expected_min_length: 8,
            must_contain_uppercase: true,
            must_contain_symbol: true,
            must_contain_number: true,
            ascii_only: true,
            no_special_characters: true,
            no_whitespaces: true,
            expected_max_length: 64
        };
        let password = Password::new(
            "Password123!\n",
            &requirements
        );
        assert!(password.check_if_password_is_valid_based_on_requirements().is_err());
        assert_eq!(
            password.check_if_password_is_valid_based_on_requirements().unwrap_err(),
            PasswordError::PasswordContainsSpecialCharacters
        );
    }

    #[test]
    fn test_password_contains_whitespace() {
        let requirements = PasswordRequirements {
            expected_min_length: 8,
            must_contain_uppercase: true,
            must_contain_symbol: true,
            must_contain_number: true,
            ascii_only: true,
            no_special_characters: true,
            no_whitespaces: true,
            expected_max_length: 64
        };
        let password = Password::new(
            "Password 123!",
            &requirements
        );
        assert!(password.check_if_password_is_valid_based_on_requirements().is_err());
        assert_eq!(
            password.check_if_password_is_valid_based_on_requirements().unwrap_err(),
            PasswordError::PasswordContainsWhitespaces
        );
    }

}