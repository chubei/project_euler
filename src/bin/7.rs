use project_euler::*;

fn main() {
    let mut n = 2;
    let mut count = 0;
    while count < 10_001 {
        if is_prime(n) {
            println!("{}", n);
            count += 1;
        }
        n += 1;
    }
    println!("{}", n - 1);
}
