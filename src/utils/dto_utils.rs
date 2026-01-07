use validator::ValidationError;


pub fn validate_name(name: &str) -> Result<(), ValidationError>{
    if name.chars().any(|c| c.is_digit(10)){ // checks for digits in decimal numerical system
        return Err(ValidationError::new("Must not contain numbers"))
    }
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn name_without_decimals_is_valid(){
        let result = validate_name("John");
        assert!(result.is_ok());
    }

    #[test]
    fn name_with_decimals_is_invalid(){
        let result = validate_name("J0hn");
        assert!(result.is_err());

        let err = result.unwrap_err();
        assert_eq!(err.code, "Must not contain numbers");
    }
}