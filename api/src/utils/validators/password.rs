use crate::errors::AppError;

/// Check password norme is respected
pub fn validate_password(password: &str) -> Result<(), AppError> {
    if password.len() < 12 {
        return Err(AppError::BadRequest("Password must be at least 12 characters".to_string()));
    }
    if !password.chars().any(|c| c.is_uppercase()) {
        return Err(AppError::BadRequest("Password must contain at least one uppercase letter".to_string()));
    }
    if !password.chars().any(|c| c.is_lowercase()) {
        return Err(AppError::BadRequest("Password must contain at least one lowercase letter".to_string()));
    }
    if !password.chars().any(|c| c.is_numeric()) {
        return Err(AppError::BadRequest("Password must contain at least one number".to_string()));
    }
    if !password.chars().any(|c| "!@#$%^&*()_+-=[]{}|;':\",./<>?".contains(c)) {
        return Err(AppError::BadRequest("Password must contain at least one special character".to_string()));
    }
    Ok(())
}
