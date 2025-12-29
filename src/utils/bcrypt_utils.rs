use bcrypt::BcryptError;
use bcrypt::DEFAULT_COST;
use bcrypt::hash;

pub struct BcryptUtils{}

impl BcryptUtils {
    pub fn make_hash(value: &str) -> Result<String, BcryptError> {
        let hashed = hash(value, DEFAULT_COST)?;
        Ok(hashed)
    }

}

