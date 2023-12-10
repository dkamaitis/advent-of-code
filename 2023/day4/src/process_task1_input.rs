pub fn process_text(contents: &str) -> u32 {
    return contents
        .split("\n")
        .map(get_card_points)
        .filter_map(|x| x)
        .sum();
}

fn get_card_points(line: &str) -> Option<u32> {
    let (winning_numbers, card_numbers) = split_card_info(line)?;
    let won_numbers: Vec<u32> = winning_numbers
        .into_iter()
        .filter(|winning_number| card_numbers.contains(winning_number))
        .collect();
    let length = won_numbers.len() as u32;
    let base: u32 = 2;
    if length > 0 {
        return Some(base.pow(length - 1));
    } else {
        return None;
    }
}

fn split_card_info(line: &str) -> Option<(Vec<u32>, Vec<u32>)> {
    let (winning_numbers, card_numbers) = line.split_once(':')?.1.split_once('|')?;
    return Some((
        str_to_vec_u32(winning_numbers),
        str_to_vec_u32(card_numbers),
    ));
}

fn str_to_vec_u32(text: &str) -> Vec<u32> {
    return text
        .trim()
        .split_whitespace()
        .map(|x| {
            x.parse::<u32>()
                .ok()
                .expect("text must be a list of space-separated numbers")
        })
        .collect();
}

#[cfg(test)]
mod tests {
    use super::{get_card_points, process_text, split_card_info, str_to_vec_u32};

    #[test]
    fn finds_correct_sum() {
        let example_input = String::from(
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53 \
            \nCard 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19 \
            \nCard 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1 \
            \nCard 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83 \
            \nCard 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36 \
            \nCard 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        );
        assert_eq!(process_text(&example_input), 13);
    }

    #[test]
    fn finds_correct_card_points() {
        let test_cases: Vec<(&str, Option<u32>)> = vec![
            ("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53 ", Some(8)),
            ("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19 ", Some(2)),
            ("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1 ", Some(2)),
            ("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83 ", Some(1)),
            ("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36 ", None),
            ("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11", None),
        ];

        for (input, expected) in test_cases {
            assert_eq!(get_card_points(input), expected);
        }
    }

    #[test]
    fn splits_card_info_correctly() {
        let test_cases: Vec<(&str, (Vec<u32>, Vec<u32>))> = vec![
            (
                "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53 ",
                (vec![41, 48, 83, 86, 17], vec![83, 86, 6, 31, 17, 9, 48, 53]),
            ),
            (
                "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19 ",
                (
                    vec![13, 32, 20, 16, 61],
                    vec![61, 30, 68, 82, 17, 32, 24, 19],
                ),
            ),
            (
                "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1 ",
                (vec![1, 21, 53, 59, 44], vec![69, 82, 63, 72, 16, 21, 14, 1]),
            ),
            (
                "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83 ",
                (
                    vec![41, 92, 73, 84, 69],
                    vec![59, 84, 76, 51, 58, 5, 54, 83],
                ),
            ),
            (
                "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36 ",
                (
                    vec![87, 83, 26, 28, 32],
                    vec![88, 30, 70, 12, 93, 22, 82, 36],
                ),
            ),
            (
                "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
                (
                    vec![31, 18, 13, 56, 72],
                    vec![74, 77, 10, 23, 35, 67, 36, 11],
                ),
            ),
        ];
        for (input, expected) in test_cases {
            assert_eq!(
                split_card_info(input).expect("should return a value for each test case"),
                expected
            );
        }
    }

    #[test]
    fn converts_str_to_vec_u32_correctly() {
        let test_cases: Vec<(&str, Vec<u32>)> = vec![
            (" 41 48 83 86 17 ", vec![41, 48, 83, 86, 17]),
            (" 13 32 20 16 61 ", vec![13, 32, 20, 16, 61]),
            ("  1 21 53 59 44 ", vec![1, 21, 53, 59, 44]),
            (" 41 92 73 84 69 ", vec![41, 92, 73, 84, 69]),
            (" 87 83 26 28 32 ", vec![87, 83, 26, 28, 32]),
            (" 31 18 13 56 72 ", vec![31, 18, 13, 56, 72]),
        ];
        for (input, expected) in test_cases {
            assert_eq!(str_to_vec_u32(input), expected);
        }
    }
}
