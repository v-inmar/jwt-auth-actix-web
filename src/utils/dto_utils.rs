use validator::ValidationError;


pub fn validate_name(name: &str) -> Result<(), ValidationError>{
    if name.chars().any(|c| c.is_digit(10)){ // checks for digits in decimal numerical system
        return Err(ValidationError::new("Must not contain numbers"))
    }
    Ok(())
}