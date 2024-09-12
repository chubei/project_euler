use project_euler::sum_of_proper_divisors;

fn main() {
    let divisor_sums = (0..10000).map(sum_of_proper_divisors).collect::<Vec<_>>();
    let result = (0..10000)
        .filter(|i| {
            let j = divisor_sums[*i as usize];
            j != *i && j < divisor_sums.len() as u32 && divisor_sums[j as usize] == *i
        })
        .sum::<u32>();
    println!("{}", result);
}
