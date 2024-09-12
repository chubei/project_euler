use itertools::Itertools;
use num_bigint::BigUint;

fn main() {
    let mut a = BigUint::from(1u32);
    let mut b = BigUint::from(2u32);
    let mut result = 4;
    let mut target = vec![0; 1000];
    target[0] = 1;
    let target = target.into_iter().join("").parse().unwrap();
    loop {
        let c = &a + &b;
        if c >= target {
            println!("{}", result);
            return;
        }
        a = b;
        b = c;
        result += 1;
    }
}
