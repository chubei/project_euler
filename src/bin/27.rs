use itertools::Itertools;
use project_euler::is_prime;

#[derive(Debug, Clone, Copy)]
struct Quadratic {
    a: i32,
    b: i32,
}

impl Quadratic {
    fn new(a: i32, b: i32) -> Self {
        Self { a, b }
    }

    fn evaluate(&self, x: i32) -> i32 {
        x * x + self.a * x + self.b
    }

    fn num_consecutive_primes(&self) -> u32 {
        let mut i = 0;
        let mut result = 0;
        loop {
            let value = self.evaluate(i);
            if value <= 0 {
                return result;
            }
            if !is_prime(value as u64) {
                return result;
            }
            i += 1;
            result += 1;
        }
    }
}

fn main() {
    let result = (-999..=999)
        .cartesian_product(-999..=999)
        .max_by_key(|(a, b)| Quadratic::new(*a, *b).num_consecutive_primes())
        .unwrap();
    println!("{}", result.0 * result.1);
}
