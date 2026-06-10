use crate::errors::AppError;

pub fn validate_login(login: &str) -> Result<(), AppError> {
    if login.len() < 3 {
        return Err(AppError::BadRequest("Login must be at least 3 characters".to_string()));
    }
    if login.len() > 30 {
        return Err(AppError::BadRequest("Login must be at most 30 characters".to_string()));
    }
    if !login.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return Err(AppError::BadRequest("Login can only contain letters, numbers and underscores".to_string()));
    }
    Ok(())
}
