use crate::errors::password_error::PasswordError; // Import custom error from the errors module

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

/// Verifies a password against the provided hash
pub fn verify_password_hash(hashed_password: &str, password: &str) -> Result<(), PasswordError> {
    match PasswordHash::new(hashed_password) {
        Ok(parsed_hash) => {
            if Argon2::default()
                .verify_password(password.as_bytes(), &parsed_hash)
                .is_ok()
            {
                Ok(())
            } else {
                Err(PasswordError::VerificationFailed)
            }
        }
        Err(_) => Err(PasswordError::ParsingFailed),
    }
}

/// Hashes a password and optionally verifies the hash against the password
pub fn hash_password(password: &str, verify: bool) -> Result<String, PasswordError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    // Hash the password
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|_| PasswordError::HashingFailed)?
        .to_string();

    // Optionally verify the hash
    if verify {
        if let Err(e) = verify_password_hash(&password_hash, password) {
            return Err(e); // Return the error if verification fails
        }
    }

    Ok(password_hash)
}
