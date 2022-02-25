fn main() {
    let mut result = 0;
    let mut a = 1;
    let mut b = 2;
    while b <= 4_000_000 {
        if b % 2 == 0 {
            result += b;
        }
        let c = a + b;
        a = b;
        b = c;
    }
    println!("{}", result);
}
