use num_bigint::BigUint;
use num_traits::{One, Zero};
use std::error::Error;
use std::fmt;

// Define a custom error type
#[derive(Debug)]
pub struct NoFactors {
    message: String,
}

impl fmt::Display for NoFactors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for NoFactors {}

// Implementing the square root function for BigUint
trait Sqrt {
    fn sqrt(&self) -> Self;
}

impl Sqrt for BigUint {
    fn sqrt(&self) -> BigUint {
        let _one = BigUint::one();
        let _two = BigUint::from(2u32);
        let mut s = self.clone();
        let mut next_s = (&s + self / &s) >> 1;
        while next_s < s {
            s = next_s;
            next_s = (&s + self / &s) >> 1;
        }
        s
    }
}

// Function to factorize a number
pub fn factorize(n: BigUint) -> Result<(BigUint, BigUint), NoFactors> {
    let two = BigUint::from(2u32);
    let sqrt_n = n.sqrt();
    let one = BigUint::one();
    let mut i = two.clone();

    while i <= sqrt_n {
        if &n % &i == Zero::zero() {
            return Ok((i.clone(), &n / &i));
        }
        i += &one; // Increment by 1
    }

    Err(NoFactors {
        message: "Cannot find factors".to_string(),
    })
}

// Function to check if a number is prime
pub fn is_prime(n: BigUint) -> bool {
    let two = BigUint::from(2u32);
    let sqrt_n = n.sqrt();
    let one = BigUint::one();
    let mut i = two.clone();

    while i <= sqrt_n {
        if &n % &i == Zero::zero() {
            return false; // Found a factor, not prime
        }
        i += &one; // Increment by 1
    }

    true // No factors found, prime
}
