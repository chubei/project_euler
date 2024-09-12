use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct NumeratorRemainder {
    numerator: u32,
    remainder: u32,
}

impl NumeratorRemainder {
    fn new(numerator: u32, remainder: u32) -> Self {
        Self {
            numerator,
            remainder,
        }
    }
}

fn unit_fraction_recurring_cycle_length(n: u32) -> u32 {
    let mut num_steps = 0u32;
    let mut pair_seen_at = HashMap::new();
    let mut numerator = 1;
    loop {
        let remainder = numerator % n;
        if remainder == 0 {
            return 0;
        }
        let pair = NumeratorRemainder::new(numerator, remainder);
        if let Some(index) = pair_seen_at.get(&pair) {
            return num_steps - *index;
        }
        pair_seen_at.insert(pair, num_steps);
        num_steps += 1;
        numerator = remainder * 10;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_unit_fraction_recurring_cycle_length() {
        assert_eq!(super::unit_fraction_recurring_cycle_length(2), 0);
        assert_eq!(super::unit_fraction_recurring_cycle_length(3), 1);
        assert_eq!(super::unit_fraction_recurring_cycle_length(6), 1);
        assert_eq!(super::unit_fraction_recurring_cycle_length(7), 6);
        assert_eq!(super::unit_fraction_recurring_cycle_length(9), 1);
    }
}

fn main() {
    let result = (1..1000)
        .max_by_key(|&n| unit_fraction_recurring_cycle_length(n))
        .unwrap();
    println!("{}", result);
}
