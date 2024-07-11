use validator::ValidationError;

pub fn validate_enabled(enabled: &bool) -> Result<(), ValidationError> {
    if *enabled {
        Ok(())
    } else {
        Err(ValidationError::new("enabled_must_be_true"))
    }
}
