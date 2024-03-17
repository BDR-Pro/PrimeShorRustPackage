use num_bigint::BigUint;
use primeshor::{factorize, is_prime};

fn main() {
    // Define the number as a string
    let n_str = "123456765343222456657333226789";
    // Convert the string to a BigUint
    let n = BigUint::parse_bytes(n_str.as_bytes(), 10).unwrap();

    match factorize(n.clone()) {
        Ok((factor1, factor2)) => println!("Factors of {}: {}, {}", n.clone(), factor1, factor2),
        Err(e) => println!("Error: {}", e),
    }

    if is_prime(n.clone()) {
        println!("{} is prime", n.clone());
    } else {
        println!("{} is not prime", n.clone());
    }
}