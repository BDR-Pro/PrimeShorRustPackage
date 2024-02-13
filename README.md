# `primeshor`

🚀 Welcome to the *coolest* Rust project on the block, `primeshor`! This isn't your grandpa's number-crunching library. We're here to make prime checking and factorization not just fast, but also *fun*.

## Quick Start

Want to dive straight into the action? Here's how you get the party started:

```rust
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
```

Run this snippet, and you'll be greeted with:

```
Factors of 100: 2, 50
100 is not prime
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