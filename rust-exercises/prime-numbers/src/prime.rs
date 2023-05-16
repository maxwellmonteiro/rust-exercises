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
