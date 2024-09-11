use std::collections::HashMap;

type Length = usize;

fn main() {
    let n = 1_000_000u32;
    let mut collatz_length = [(1, 1)].into_iter().collect::<HashMap<u32, Length>>();
    for i in 2..n {
        get_collatz_length(i, &mut collatz_length);
    }
    let result = collatz_length
        .iter()
        .filter(|(key, _)| **key < n)
        .max_by_key(|(_, value)| *value)
        .unwrap()
        .0;
    println!("{}", result);
}

fn get_collatz_length(n: u32, cache: &mut HashMap<u32, Length>) -> Length {
    if let Some(length) = cache.get(&n) {
        return *length;
    };
    let result = if n % 2 == 0 {
        get_collatz_length(n / 2, cache)
    } else {
        get_collatz_length(3 * n + 1, cache)
    } + 1;
    cache.insert(n, result);
    result
}
