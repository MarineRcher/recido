use bcrypt::{hash, verify, DEFAULT_COST};
use crate::errors::AppError;


/// Hashes a plaintext password using bcrypt.
pub fn hash_password(password: &str) -> Result<String, AppError> {
    hash(password, DEFAULT_COST).map_err(|_| AppError::InternalError("Failed to hash password".to_string()))
}

/// Verifies a plaintext password against a bcrypt hash.
pub fn verify_password(password: &str, hash: &str) -> Result<bool, AppError> {
    verify(password, hash).map_err(|_| AppError::InternalError("Failed to verify password".to_string()))
}
