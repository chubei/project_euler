use std::collections::HashSet;

use num_bigint::BigUint;

fn main() {
    let mut numbers = HashSet::new();
    for a in 2..=100u32 {
        for b in 2..=100 {
            numbers.insert(BigUint::from(a).pow(b));
        }
    }
    println!("{}", numbers.len());
}
