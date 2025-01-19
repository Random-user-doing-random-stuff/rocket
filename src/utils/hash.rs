#![allow(dead_code)]

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

/// Verifies a password against the provided hash
pub fn verify_password_hash(hashed_password: &str, password: &str) {
    let parsed_hash = PasswordHash::new(hashed_password).expect("Failed to parse password hash");

    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .expect("Failed to verify password");
}

/// Hashes a password and optionally verifies the hash against the password
pub fn hash_password(password: &str, verify: bool) -> String {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    // Hash the password
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .expect("Error hashing password")
        .to_string();

    // Optionally verify the hash
    if verify {
        verify_password_hash(&password_hash, password);
    }

    password_hash
}
