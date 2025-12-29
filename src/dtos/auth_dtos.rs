use serde::Serialize;
use serde::Deserialize;
use validator::Validate;


use crate::utils::dto_utils::validate_name;
use crate::constants::FIRSTNAME_MAX_LENGTH;
use crate::constants::FIRSTNAME_MIN_LENGTH;
use crate::constants::LASTNAME_MAX_LENGTH;
use crate::constants::LASTNAME_MIN_LENGTH;
use crate::constants::PASSWORD_MIN_LENGTH;
use crate::constants::PASSWORD_MAX_LENGTH;


#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct AuthRegisterDto{
    #[validate(length(min=FIRSTNAME_MIN_LENGTH, max=FIRSTNAME_MAX_LENGTH), custom(function="validate_name"))]
    pub firstname: String,

    #[validate(length(min=LASTNAME_MIN_LENGTH, max=LASTNAME_MAX_LENGTH), custom(function="validate_name"))]
    pub lastname: String,

    #[validate(email)]
    pub email: String,

    #[validate(length(min = PASSWORD_MIN_LENGTH, max = PASSWORD_MAX_LENGTH))]
    pub password: String,
    
    // this will be checked within the handler
    #[validate(length(min = 8, max = 255))]
    pub repeat: String,
}

#[derive(Debug,Serialize,Deserialize, Validate)]
pub struct AuthLoginDto {
    pub email: String,
    pub password: String,
}