use project_euler::sum_of_proper_divisors;

fn is_abundant(n: u32) -> bool {
    sum_of_proper_divisors(n) > n
}

fn main() {
    let is_abundant = (0..28124).map(is_abundant).collect::<Vec<_>>();
    let is_sum_of_two_abundant = |n: u32| {
        for i in 0..=n / 2 {
            if is_abundant[i as usize] && is_abundant[(n - i) as usize] {
                return true;
            }
        }
        false
    };
    let result = (0..28124)
        .filter(|&n| !is_sum_of_two_abundant(n))
        .sum::<u32>();
    println!("{}", result);
}
