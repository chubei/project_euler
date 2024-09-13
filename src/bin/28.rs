fn main() {
    let mut result = 0;
    for side_length in (3..=1001).step_by(2) {
        let upper_right = side_length * side_length;
        for i in 0..4 {
            result += upper_right - (side_length - 1) * i;
        }
    }
    result += 1;
    println!("{}", result);
}
