use core::f32::{INFINITY, NEG_INFINITY};
use std::collections::HashMap;

pub fn process_text(contents: &str) -> u32 {
    return contents
        .split("\n")
        .map(first_and_last_digits)
        .filter_map(|x| x)
        .sum();
}

pub fn first_and_last_digits(text: &str) -> Option<u32> {
    let mut first_index: i64 = INFINITY as i64;
    let mut first_digit: Option<&str> = None;
    let mut last_index: i64 = NEG_INFINITY as i64;
    let mut last_digit: Option<&str> = None;

    const DIGITS: [&str; 18] = [
        "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six",
        "seven", "eight", "nine",
    ];

    for digit_string in DIGITS.iter() {
        let first_i = match text.find(digit_string) {
            Some(index) => index as i64,
            None => continue,
        };
        let last_i = match text.rfind(digit_string) {
            Some(index) => index as i64,
            None => continue,
        };
        if first_i < first_index {
            first_index = first_i;
            first_digit = Some(digit_string);
        }
        if last_i > last_index {
            last_index = last_i;
            last_digit = Some(digit_string);
        }
    }
    return Some((str_digit_to_int(first_digit?)? * 10 + str_digit_to_int(last_digit?)?) as u32);
}

pub fn str_digit_to_int(digit: &str) -> Option<u32> {
    let digit_map: HashMap<_, _> = vec![
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]
    .into_iter()
    .collect();
    if digit.chars().count() == 1 {
        return digit.parse::<u32>().ok();
    } else {
        return digit_map.get(digit).cloned();
    }
}

#[cfg(test)]
mod tests {
    use super::{first_and_last_digits, process_text};

    #[test]
    fn finds_correct_sum() {
        let example_input = String::from(
            "two1nine\neightwothree\nabcone2threexyz \
            \nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen",
        );
        assert_eq!(process_text(&example_input), 281);
    }

    #[test]
    fn finds_first_and_last_digits() {
        let test_cases = vec![
            ("two1nine", 29),
            ("eightwothree", 83),
            ("abcone2threexyz", 13),
            ("xtwone3four", 24),
            ("4nineeightseven2", 42),
            ("zoneight234", 14),
            ("7pqrstsixteen", 76),
        ];
        for (input, expected) in test_cases {
            assert_eq!(
                first_and_last_digits(input).expect("should return a value for test cases"),
                expected
            );
        }
    }
}
