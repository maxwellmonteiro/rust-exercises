mod prime;

use std::ops::RangeInclusive;
fn main() {
    let primes: Vec<u32> = get_primes_sieve(100_000_000);

    // print_primes(100);
    // print_primes_sieve(100);

    println!("primes: {}", primes.iter().count());

}

fn print_primes(end: u32) {
    let primes: Vec<u32> = get_primes(end);

    let result: String = primes.iter()
        .map(|v| v.to_string())
        .collect::<Vec<String>>()
        .join(", ");
    
    println!("Primes: {}", result);
}

fn print_primes_sieve(end: u32) {
    let primes: Vec<u32> = get_primes_sieve(end);

    let result: String = primes.iter()
        .map(|v| v.to_string())
        .collect::<Vec<String>>()
        .join(", ");
    
    println!("Primes: {}", result);
}

fn get_primes(end: u32) -> Vec<u32> {
    let numbers: RangeInclusive<u32> = RangeInclusive::new(2, end);

    return numbers
        .into_iter()
        .filter(|n: &u32| prime::is_prime(n))
        .collect();
}

fn get_primes_sieve(end: u32) -> Vec<u32> {
    return prime::get_primes_sieve(end)
        .into_iter()
        .collect();
}

