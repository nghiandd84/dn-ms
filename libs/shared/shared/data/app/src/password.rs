use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

use shared_shared_data_error::password::PasswordError;

pub fn hash(password: impl Into<String>) -> Result<String, PasswordError> {
    let password = password.into();

    if password.is_empty() {
        return Err(PasswordError::EmptyPassword);
    }

    let salt = SaltString::generate(&mut OsRng);
    let hashed_password = Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map_err(|_| PasswordError::HashingError)?
        .to_string();

    Ok(hashed_password)
}

pub fn compare(password: &str, hashed_password: &str) -> Result<bool, PasswordError> {
    if password.is_empty() {
        return Err(PasswordError::EmptyPassword);
    }

    let parsed_hash =
        PasswordHash::new(hashed_password).map_err(|_| PasswordError::InvalidHashFormat)?;

    let password_matches = Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .map_or(false, |_| true);

    Ok(password_matches)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_success() {
        let password = "mysecretpassword";
        let hashed = hash(password);
        assert!(hashed.is_ok());
        let hashed = hashed.unwrap();
        assert!(!hashed.is_empty());
    }

    #[test]
    fn test_hash_empty_password() {
        let result = hash("");
        assert!(matches!(result, Err(PasswordError::EmptyPassword)));
    }

    #[test]
    fn test_compare_success() {
        let password = "mysecretpassword";
        let hashed = hash(password).unwrap();
        let result = compare(password, &hashed);
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[test]
    fn test_compare_wrong_password() {
        let password = "mysecretpassword";
        let wrong_password = "wrongpassword";
        let hashed = hash(password).unwrap();
        let result = compare(wrong_password, &hashed);
        assert!(result.is_ok());
        assert!(!result.unwrap());
    }

    #[test]
    fn test_compare_empty_password() {
        let password = "";
        let hashed = hash("something").unwrap();
        let result = compare(password, &hashed);
        assert!(matches!(result, Err(PasswordError::EmptyPassword)));
    }

    #[test]
    fn test_compare_invalid_hash() {
        let password = "password";
        let invalid_hash = "not_a_valid_hash";
        let result = compare(password, invalid_hash);
        assert!(matches!(result, Err(PasswordError::InvalidHashFormat)));
    }
}
