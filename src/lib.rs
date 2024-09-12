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

use std::collections::HashMap;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct PrimeFactorization {
    pub map: HashMap<u64, u32>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PrimeFactor {
    pub prime: u64,
    pub power: u32,
}

impl PrimeFactorization {
    pub fn add_prime_factor(&mut self, prime: u64, power: u32) {
        use std::collections::hash_map::Entry;
        match self.map.entry(prime) {
            Entry::Occupied(mut entry) => *entry.get_mut() += power,
            Entry::Vacant(entry) => {
                entry.insert(power);
            }
        }
    }

    pub fn remove_prime_factor(&mut self, prime: u64, power: u32) {
        let current = self.map.get_mut(&prime).unwrap();
        *current = current.checked_sub(power).unwrap();
    }

    pub fn merge(&mut self, other: &PrimeFactorization) {
        for (prime, power) in &other.map {
            self.add_prime_factor(*prime, *power);
        }
    }

    pub fn num_factors(&self) -> u32 {
        self.map.values().map(|power| power + 1).product()
    }

    pub fn value(&self) -> u64 {
        self.map
            .iter()
            .map(|(prime, power)| prime.pow(*power))
            .product()
    }

    pub fn into_vec(self) -> Vec<PrimeFactor> {
        self.map
            .into_iter()
            .map(|(prime, power)| PrimeFactor { prime, power })
            .collect()
    }
}

pub fn prime_factorization(mut n: u64) -> PrimeFactorization {
    let mut result = Default::default();

    if n == 1 {
        result
    } else if n == 2 || n == 3 {
        result.add_prime_factor(n, 1);
        result
    } else {
        while n > 3 {
            let mut reduced = false;
            let upper_factor = (n as f64).sqrt().floor() as u64;
            for i in 2..=upper_factor {
                if n % i == 0 {
                    result.add_prime_factor(i, 1);
                    n /= i;
                    reduced = true;
                    break;
                }
            }
            if !reduced {
                // This is a prime.
                result.add_prime_factor(n, 1);
                n = 1;
            }
        }

        if n == 3 {
            result.add_prime_factor(3, 1);
        } else if n == 2 {
            result.add_prime_factor(2, 1);
        }

        result
    }
}

pub fn sieve(n: usize) -> Vec<usize> {
    let mut is_prime = vec![true; n + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    for i in 2..=(n as f64).sqrt().ceil() as usize {
        let mut sieved = i * 2;
        while sieved <= n {
            is_prime[sieved] = false;
            sieved += i;
        }
    }
    is_prime
        .into_iter()
        .enumerate()
        .filter_map(|(i, is_prime)| if is_prime { Some(i) } else { None })
        .collect()
}

pub fn sum_of_proper_divisors(n: u32) -> u32 {
    (1..n).filter(|i| n % i == 0).sum()
}

pub mod grid;
pub mod path_sum;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primie_factorization() {
        assert_eq!(prime_factorization(1).into_vec(), vec![]);
        assert_eq!(
            prime_factorization(2).into_vec(),
            vec![PrimeFactor { prime: 2, power: 1 }]
        );
        assert_eq!(
            prime_factorization(3).into_vec(),
            vec![PrimeFactor { prime: 3, power: 1 }]
        );
        assert_eq!(
            prime_factorization(4).into_vec(),
            vec![PrimeFactor { prime: 2, power: 2 }]
        );
        let mut prime_factors_500 = prime_factorization(500).into_vec();
        // A prime factor + its multiplicity doesn't have a clear ordering, but let's choose one,
        // as otherwise the test output is nondeterministic due to hashing.
        prime_factors_500.sort_by(|fa, fb| fa.prime.cmp(&fb.prime));
        assert_eq!(
            prime_factors_500,
            vec![
                PrimeFactor { prime: 2, power: 2 },
                PrimeFactor { prime: 5, power: 3 }
            ]
        );
    }

    #[test]
    fn test_sieve() {
        assert_eq!(sieve(11), [2, 3, 5, 7, 11])
    }
}
