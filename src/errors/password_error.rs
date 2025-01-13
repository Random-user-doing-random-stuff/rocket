use std::fmt;

/// Custom error type for password hashing and verification
#[derive(Debug)]
pub enum PasswordError {
    HashingFailed,
    ParsingFailed,
    VerificationFailed,
}

impl fmt::Display for PasswordError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PasswordError::HashingFailed => write!(f, "Failed to hash the password."),
            PasswordError::ParsingFailed => write!(f, "Failed to parse the hashed password."),
            PasswordError::VerificationFailed => write!(f, "Password verification failed."),
        }
    }
}

impl std::error::Error for PasswordError {}
