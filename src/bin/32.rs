use std::collections::HashSet;

fn is_pandigital(a: u32, b: u32) -> bool {
    let mut digits = [false; 10];
    let mut check = |mut n| {
        while n > 0 {
            let digit = n % 10;
            if digit == 0 || digits[digit as usize] {
                return false;
            }
            digits[digit as usize] = true;
            n /= 10;
        }
        true
    };
    check(a) && check(b) && check(a * b) && digits[1..].iter().all(|&x| x)
}

fn main() {
    let result = (1..=31622)
        .flat_map(|i| (i + 1..=31622).map(move |j| (i, j)))
        .filter_map(|(a, b)| {
            if is_pandigital(a, b) {
                Some(a * b)
            } else {
                None
            }
        })
        .collect::<HashSet<_>>()
        .iter()
        .sum::<u32>();
    println!("{}", result);
}
