use std::error::Error;
use std::fmt;

// Define a custom error type
#[derive(Debug)]
pub struct MyCustomException {
    message: String,
}

impl fmt::Display for MyCustomException {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for MyCustomException {}

// Function to factorize a number
pub fn factorize(n: u64) -> Result<(u64, u64), MyCustomException> {
    let sqrt_n = (n as f64).sqrt() as u64;
    for i in 2..=sqrt_n {
        if n % i == 0 {
            return Ok((i, n / i));
        }
    }
    Err(MyCustomException {
        message: "Cannot find factors".to_string(),
    })
}

// Function to check if a number is prime
pub fn is_prime(n: u64) -> bool {
    let sqrt_n = (n as f64).sqrt() as u64;
    for i in 2..=sqrt_n {
        if n % i == 0 {
            return false; // Found a factor, not prime
        }
    }
    true // No factors found, prime
}

