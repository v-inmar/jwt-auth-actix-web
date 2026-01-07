use rand::Rng;
use rand::distr::Alphanumeric;

pub struct StringUtils {}

impl StringUtils {
    pub fn generate_random_string(length: usize) -> String {
        rand::rng() // use a random generator
        .sample_iter(&Alphanumeric) // use alphanumeric distribution
        .take(length) // limit
        .map(|b| char::from(b)) // converts byte into char
        .collect() // into String
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_string_of_correct_length() {
        let short = StringUtils::generate_random_string(5);
        assert_eq!(short.len(), 5);

        let long = StringUtils::generate_random_string(20);
        assert_eq!(long.len(), 20);
    }

    #[test]
    fn generates_only_alphanumeric_characters() {
        let random = StringUtils::generate_random_string(100);
        
        for c in random.chars() {
            assert!(c.is_alphanumeric(), "Found non-alphanumeric char: {}", c);
        }
    }

    #[test]
    fn zero_length_returns_empty_string() {
        let empty = StringUtils::generate_random_string(0);
        assert_eq!(empty, "");
    }
}
