pub fn process_text(contents: &str) -> u32 {
    return contents
        .split("\n")
        .map(first_and_last_digits)
        .filter_map(|x| x)
        .sum();
}

pub fn first_and_last_digits(text: &str) -> Option<u32> {
    let first_digit = match get_first_digit(text.chars()) {
        Some(digit) => digit.to_digit(10),
        None => None,
    };
    let last_digit = match get_first_digit(text.chars().rev()) {
        Some(digit) => digit.to_digit(10),
        None => None,
    };

    return Some((first_digit? * 10 + last_digit?) as u32);
}

pub fn get_first_digit<I>(text: I) -> Option<char>
where
    I: Iterator<Item = char>,
{
    for c in text {
        if c.is_digit(10) {
            return Some(c);
        }
    }
    return None;
}

#[cfg(test)]
mod tests {
    use super::{first_and_last_digits, get_first_digit, process_text};

    #[test]
    fn finds_correct_sum() {
        let example_input = String::from("1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet");
        assert_eq!(process_text(&example_input), 142);
    }

    #[test]
    fn finds_first_and_last_digits() {
        let test_cases = vec![
            ("1abc2", 12),
            ("pqr3stu8vwx", 38),
            ("a1b2c3d4e5f", 15),
            ("treb7uchet", 77),
        ];
        for (input, expected) in test_cases {
            assert_eq!(first_and_last_digits(input).unwrap(), expected);
        }
    }

    #[test]
    fn finds_first_digit() {
        let test_cases = vec![
            ("1abc2", '1'),
            ("pqr3stu8vwx", '3'),
            ("a1b2c3d4e5f", '1'),
            ("treb7uchet", '7'),
        ];
        for (input, expected) in test_cases {
            assert_eq!(get_first_digit(input.chars()).unwrap(), expected);
        }
    }
}
