use project_euler::*;

fn main() {
    let mut current_factorization = prime_factorization(1);
    let mut next = 2;
    let mut next_factorization = prime_factorization(next);
    loop {
        current_factorization.merge(&next_factorization);
        current_factorization.remove_prime_factor(2, 1);
        if current_factorization.num_factors() > 500 {
            println!("{}", current_factorization.value());
            break;
        }

        current_factorization = next_factorization;
        next += 1;
        next_factorization = prime_factorization(next);
    }
}
