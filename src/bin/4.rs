use project_euler::*;

fn main() {
    let mut result = 0;
    for i in 100..1000 {
        for j in i..1000 {
            let product = i * j;
            if is_palindromic(product) {
                result = result.max(product);
            }
        }
    }
    println!("{}", result);
}
