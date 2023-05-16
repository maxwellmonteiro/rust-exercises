mod prime;

use std::ops::RangeInclusive;
fn main() {
    let numbers: RangeInclusive<u32> = RangeInclusive::new(1, 100_000_000);

    let result: usize = numbers.into_iter()
        .filter(|n: &u32| prime::is_prime(n))
        .map(|n: u32| n.to_string())
        .count();

    println!("Primes: {}", result);
}

