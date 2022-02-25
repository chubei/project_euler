fn main() {
    let mut nums: Vec<_> = (2..21).collect();
    let mut result = 1;
    let mut index = 0;
    while index < nums.len() {
        let num = nums[index];
        result *= num;
        index += 1;
        for n in &mut nums[index..] {
            if *n % num == 0 {
                *n /= num;
            }
        }
    }
    println!("{}", result);
}
