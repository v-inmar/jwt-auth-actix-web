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




#[cfg(test)]
mod tests {
    use super::*;
    use bcrypt::verify;

    #[test]
    fn make_hash_returns_valid_bcrypt_hash() {
        let password = "my_password";

        let hashed = BcryptUtils::make_hash(password)
            .expect("hashing should succeed");

        assert!(!hashed.is_empty());
        assert!(verify(password, &hashed).expect("verify should succeed"));
    }

    #[test]
    fn make_hash_does_not_match_wrong_password() {
        let password = "correct_password";
        let wrong_password = "wrong_password";

        let hashed = BcryptUtils::make_hash(password)
            .expect("hashing should succeed");

        let is_valid = verify(wrong_password, &hashed)
            .expect("verify should not error");

        assert!(!is_valid, "wrong password must not validate against hash");
    }


    #[test]
    fn verify_fails_with_invalid_hash_format() {
        let password = "password";
        let bad_hash = "this-is-not-a-valid-bcrypt-hash";

        let result = bcrypt::verify(password, bad_hash);

        assert!(result.is_err(), "verify should error on malformed hash");
    }
}

