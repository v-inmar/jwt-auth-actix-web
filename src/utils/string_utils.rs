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