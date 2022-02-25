pub fn get_smallest_prime_factor(n: u64) -> u64 {
    for i in 2..=(n / 2) {
        if n % i == 0 {
            return i;
        }
    }
    n
}

pub fn is_prime(n: u64) -> bool {
    get_smallest_prime_factor(n) == n
}

pub fn is_palindromic(n: u64) -> bool {
    is_palindromic_chars(number_to_reverse_chars(n))
}

pub fn number_to_reverse_chars(mut n: u64) -> Vec<u8> {
    let mut result = vec![];
    while n > 0 {
        result.push((n % 10) as u8);
        n /= 10;
    }
    result
}

pub fn is_palindromic_chars(chars: Vec<u8>) -> bool {
    let end = chars.len() / 2 + 1;
    for i in 0..end {
        if chars[i] != chars[chars.len() - i - 1] {
            return false;
        }
    }
    true
}
