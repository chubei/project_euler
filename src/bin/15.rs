fn main() {
    let n = 20;
    let mut dp = vec![vec![0usize; n + 1]; n + 1];
    dp[0][0] = 1;
    for i in 0..=n {
        for j in 0..=n {
            if i > 0 {
                dp[i][j] += dp[i - 1][j];
            }
            if j > 0 {
                dp[i][j] += dp[i][j - 1];
            }
        }
    }
    println!("{}", dp[n][n]);
}
