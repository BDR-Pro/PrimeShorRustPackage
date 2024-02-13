use primeshor::{factorize, is_prime};

fn main() {
    let n = 100;
    match factorize(n) {
        Ok((factor1, factor2)) => println!("Factors of {}: {}, {}", n, factor1, factor2),
        Err(e) => println!("Error: {}", e),
    }

    if is_prime(n) {
        println!("{} is prime", n);
    } else {
        println!("{} is not prime", n);
    }
}