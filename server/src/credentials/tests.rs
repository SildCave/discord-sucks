#[cfg(test)]
mod tests {
    use crate::credentials::{Password, PasswordError, PasswordRequirements};

    use pretty_assertions::assert_eq;

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