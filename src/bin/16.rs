use num_bigint::BigUint;

fn main() {
    let result = BigUint::from(2u32).pow(1000);
    println!(
        "{}",
        result
            .to_string()
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .sum::<u32>()
    );
}
