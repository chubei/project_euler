use itertools::Itertools;

fn main() {
    println!(
        "{}",
        (0..=9)
            .permutations(10)
            .nth(1_000_000 - 1)
            .unwrap()
            .into_iter()
            .join("")
    );
}
