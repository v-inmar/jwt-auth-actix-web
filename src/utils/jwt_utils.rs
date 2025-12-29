use std::env;

use serde::Serialize;
use serde::Deserialize;

use chrono::DateTime;
use chrono::Utc;
use chrono::Duration;

use jsonwebtoken::encode;
use jsonwebtoken::decode;
use jsonwebtoken::EncodingKey;
use jsonwebtoken::DecodingKey;
use jsonwebtoken::Header;
use jsonwebtoken::TokenData;
use jsonwebtoken::Validation;

pub enum TokenType {
    Access,
    Refresh
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,        // Subject (the auth token)
    pub exp: i64,           // Expiration time
    pub iat: i64,           // Issued at time
    pub token_type: String, // access or refresh
}

pub struct JwtUtils {}

impl JwtUtils {

    /// Generate json web token
    fn _gen_jwt(sub: &str, token_type: &TokenType, exp: &DateTime<Utc>, secret: &str) -> Result<String, Box<dyn std::error::Error>>{
        let token_type_string = match token_type {
            TokenType::Access => "access",
            TokenType::Refresh => "refresh"
        };

        let claims = Claims {
            sub: sub.to_string(),
            exp: exp.timestamp(),
            iat: Utc::now().timestamp(),
            token_type: token_type_string.to_string(),
        };

        let jwt_token = encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_bytes()))?;

        Ok(jwt_token)
    }

    /// Decode the json web token
    fn _decode_jwt(token: &str, secret: &str) -> Result<TokenData<Claims>, Box<dyn std::error::Error>>{
        let mut validation = Validation::new(Header::default().alg);

        validation.validate_exp = true; // The expiration date will be checked

        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(secret.as_ref()),
            &validation,
        )?;

        Ok(token_data)
    }

    /// Generate access token
    pub fn gen_access_token(sub_value: &str) -> Result<String, Box<dyn std::error::Error>>{
        // get secret
        let access_secret = env::var("JWT_ACCESS_SECRET")
            .map_err(|e| {
                Box::new(e)
            })?;


        // get exp time
        let exp_minutes = env::var("ACCESS_TOKEN_EXPIRATION_MINUTES")
            .map_err(|e| {
                Box::new(e)
            })?
            .parse::<i64>()
            .map_err(|e| {
                Box::new(e)
            })?;
        
        // set the expiration time to be used into the token
        let utc_now = Utc::now();
        let expiration = utc_now + Duration::minutes(exp_minutes);

        let access_token = JwtUtils::_gen_jwt(sub_value, &TokenType::Access, &expiration, &access_secret)?;

        Ok(access_token)
    }


    /// Generate refresh token
    pub fn gen_refresh_token(sub_value: &str) -> Result<String, Box<dyn std::error::Error>>{
        // get secret
        let refresh_secret = env::var("JWT_REFRESH_SECRET")
            .map_err(|e| {
                Box::new(e)
            })?;


        // get exp time
        let exp_days = env::var("REFRESH_TOKEN_EXPIRATION_DAYS")
            .map_err(|e| {
                Box::new(e)
            })?
            .parse::<i64>()
            .map_err(|e| {
                Box::new(e)
            })?;
        
        // set the expiration time to be used into the token
        let utc_now = Utc::now();
        let expiration = utc_now + Duration::days(exp_days);

        let refresh_token = JwtUtils::_gen_jwt(sub_value, &TokenType::Refresh, &expiration, &refresh_secret)?;

        Ok(refresh_token)
    }


    pub fn decode_access_token(token: &str) -> Result<TokenData<Claims>, Box<dyn std::error::Error>>{
        let secret = env::var("JWT_ACCESS_SECRET").map_err(|e| {
            Box::new(e)
        })?;

        let token_data = JwtUtils::_decode_jwt(&token, &secret)?;

        Ok(token_data)
    }

    pub fn decode_refresh_token(token: &str) -> Result<TokenData<Claims>, Box<dyn std::error::Error>>{
        let secret = env::var("JWT_REFRESH_SECRET").map_err(|e| {
            Box::new(e)
        })?;

        let token_data = JwtUtils::_decode_jwt(&token, &secret)?;

        Ok(token_data)
    } 

}