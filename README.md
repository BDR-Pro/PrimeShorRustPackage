# `primeshor`

🚀 Welcome to the *coolest* Rust project on the block, `primeshor`! This isn't your grandpa's number-crunching library. We're here to make prime checking and factorization not just fast, but also *fun*.

`But wait, what's so fun about prime numbers?"`you ask. Well, we're here to show you that prime numbers are more than just the building blocks of cryptography. They're the superheroes of the number world, and we're here to celebrate them in all their glory.

## Updated `0.2.0` to handle 🚀 `BigUint` for massive numbers by parsing from a string, ensuring it's set for operations beyond usual integer bounds! ✨

## Quick Start

Want to dive straight into the action? Here's how you get the party started:

```rust
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
```

Run this snippet, and you'll be greeted with:

```bash

Factors of 123456765343222456657333226789: 3529, 34983498255376156604514941
123456765343222456657333226789 is not prime

```

## What's Shor's Algorithm?

Now, let's get a tiny bit *nerdy*. Ever heard of Shor's Algorithm? It's like the superhero of algorithms when it comes to breaking down numbers into their prime factors. 🦸‍♂️

Shor's Algorithm is a quantum computing algorithm for integer factorization. Translated into Gen Z speak: it's a super-fast way to find out what numbers multiply together to make your original number, but it uses qubits instead of bits, making it kinda like your quantum BFF for cryptography.

### How Fast Are We Talking?

In the traditional computing world, factorizing large numbers takes a *long* time (we're talking more than your patience level for sure). But with Shor's Algorithm, you can factorize these massive numbers in polynomial time. That's right, it runs in `O((log n)^2 (log log n) (log log log n))` time, making it exponentially faster than the best-known algorithms running on classical computers.

## Why `primeshor`?

While we're not running on quantum computers (yet 😉), `primeshor` aims to bring a bit of that excitement into the Rust ecosystem. Whether you're a math enthusiast, a budding cryptographer, or just here for the quantum revolution, `primeshor` is your go-to library for exploring prime numbers and the wonders of factorization.

So, clone the repo, dive into the docs, and start experimenting. Who knows? Maybe you'll be the one to simulate Shor's Algorithm on a classical computer (or at least have fun trying).

---

Remember, `primeshor` is all about making the complex simple and the mundane fun. Happy coding! 🎉
