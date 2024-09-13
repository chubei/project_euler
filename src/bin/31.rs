fn main() {
    let coins = [1, 2, 5, 10, 20, 50, 100, 200];
    let mut dp = vec![vec![0; coins.len()]; 201];
    dp[0].fill(1);
    dp.iter_mut().for_each(|row| row[0] = 1);
    for value in 1..=200 {
        for (i, &coin) in coins.iter().enumerate().skip(1) {
            for num_coin in 0..=value / coin {
                dp[value][i] += dp[value - num_coin * coin][i - 1];
            }
        }
    }
    println!("{}", dp.last().unwrap().last().unwrap());
}
