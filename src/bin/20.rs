use num_bigint::BigUint;

fn main() {
    let mut result = BigUint::from(1u32);
    for i in 1..=100u32 {
        result *= i;
    }
    let sum = result
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .sum::<u32>();
    println!("{}", sum);
}
