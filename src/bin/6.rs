fn main() {
    let mut sum = 0;
    let mut square_sum = 0;
    for n in 1..=100 {
        sum += n;
        square_sum += n * n;
    }
    println!("{}", sum * sum - square_sum);
}
