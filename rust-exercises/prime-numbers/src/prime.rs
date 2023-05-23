struct PrimeCheck {
    value: u32,
    is_prime: bool
}

pub fn is_prime(value: &u32) -> bool {
    if *value == 1 {
        return false;
    }
    let sqrt = (*value as f64).sqrt() as u32;
    
    for n in 2..=sqrt {
        if (value % n) == 0 {
            return false;
        }
    }
    return true;
}

pub fn get_primes_sieve(end: u32) -> Vec<u32> {
    let mut values: Vec<PrimeCheck> = Vec::with_capacity(((end) + 1) as usize);

    let begin: u32 = 2;

    (begin..=end).for_each(|v| values.push(PrimeCheck{value: v, is_prime: true}));

    let mut begin_value: u32 = (begin as f64).sqrt() as u32;
    let end_value: u32 = (end as f64).sqrt() as u32;

    begin_value = if begin_value == 1 { 2 } else { begin_value };
    while begin_value <= end_value {        
        let mut test_value: u32 = u32::from(begin_value);
        while test_value < end {
            test_value += begin_value;
            let idx = (test_value - begin) as usize;
            if idx < values.len() {
                values[idx].is_prime = false;
            }
        }
        begin_value += 1;
    } 
    return values.into_iter()
        .filter(|v| v.is_prime)
        .map(|v| v.value)
        .collect();
}