use project_euler::*;

fn main() {
    let mut n = 600851475143;
    let mut factor = get_smallest_prime_factor(n);
    while factor != n {
        n /= factor;
        factor = get_smallest_prime_factor(n);
    }
    println!("{}", factor);
}
