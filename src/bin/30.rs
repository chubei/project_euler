fn digit_fifth_power_sum(mut n: u32) -> u32 {
    let mut result = 0;
    while n > 0 {
        result += (n % 10).pow(5);
        n /= 10;
    }
    result
}

fn main() {
    println!(
        "{}",
        (10..1_000_000)
            .filter(|&x| x == digit_fifth_power_sum(x))
            .sum::<u32>()
    );
}
