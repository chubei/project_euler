use project_euler::sieve;

fn main() {
    let primes = sieve(2_000_000);
    println!("{}", primes.into_iter().sum::<usize>());
}
