use validator::{Validate, ValidationError};

pub fn validate_unoffensive_name(name: &str) -> Result<(), ValidationError> {
    // Todo: expand upon validation rules
    if name.contains("slut") {
        return Err(ValidationError::new("Terrible taste in name"));
    }
    if name.contains("bitch") {
        return Err(ValidationError::new("Terrible taste in name"));
    }
    if name.contains("fuck") {
        return Err(ValidationError::new("Terrible taste in name"));
    }
    if name.contains("faggot") {
        return Err(ValidationError::new("Terrible taste in name"));
    }
    if name.contains("cunt") {
        return Err(ValidationError::new("Terrible taste in name"));
    }
    if name.contains("nigger") {
        return Err(ValidationError::new("Terrible taste in name"));
    }
    Ok(())
}
