use crate::errors::AppError;

/// Check if we have an email field
pub fn validate_email(email: &str) -> Result<(), AppError> {
    let has_at = email.contains('@');
    let parts: Vec<&str> = email.split('@').collect();
    let valid = has_at
        && parts.len() == 2
        && !parts[0].is_empty()
        && parts[1].contains('.')
        && !parts[1].starts_with('.')
        && !parts[1].ends_with('.');

    if !valid {
        return Err(AppError::BadRequest("Invalid email format".to_string()));
    }
    Ok(())
}
