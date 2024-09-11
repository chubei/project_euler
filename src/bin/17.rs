fn main() {
    let one_to_nine = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let ten_to_nineteen = [
        "ten",
        "eleven",
        "twelve",
        "thirteen",
        "fourteen",
        "fifteen",
        "sixteen",
        "seventeen",
        "eighteen",
        "nineteen",
    ];
    let twenty_to_ninety = [
        "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
    ];

    let mut result = 0;
    // one to nine
    result += one_to_nine.iter().map(|s| s.len()).sum::<usize>();
    // ten to nineteen
    result += ten_to_nineteen.iter().map(|s| s.len()).sum::<usize>();
    // 20 to 99
    for ten in twenty_to_ninety.iter() {
        result += one_to_nine
            .iter()
            .map(|s| ten.len() + s.len())
            .sum::<usize>()
            + ten.len();
    }
    let one_to_ninety_nine = result;
    let hundreds = one_to_nine
        .iter()
        .map(|s| s.len() + "hundred".len())
        .collect::<Vec<_>>();
    result += hundreds
        .iter()
        .map(|hundred_len| hundred_len + (hundred_len + "and".len()) * 99 + one_to_ninety_nine)
        .sum::<usize>();
    result += "onethousand".len();
    println!("{}", result);
}
