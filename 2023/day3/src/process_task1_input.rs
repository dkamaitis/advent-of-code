pub fn process_text(contents: &str) -> u32 {
    let contents_vec: Vec<Vec<char>> = contents.split("\n").map(|x| x.chars().collect()).collect();

    let engine_part_numbers: Vec<u32> = find_engine_part_numbers(contents_vec);
    return engine_part_numbers.iter().sum();
}

fn find_engine_part_numbers(contents: Vec<Vec<char>>) -> Vec<u32> {
    let contents: Vec<Vec<char>> = contents.into_iter().filter(|x| x.len() > 0).collect();
    let mut engine_part_numbers: Vec<u32> = vec![];

    let m: usize = contents.len();
    let mut n: usize;

    let mut adjacent_indices: Vec<(usize, usize)>;
    let mut current_char: char;
    let mut current_char_has_adjacent_symbols: bool;
    let mut current_digits: Vec<char>;
    let mut current_digits_have_adjacent_symbols: bool;

    for i in 0..m {
        current_digits = vec![];
        current_digits_have_adjacent_symbols = false;
        n = contents[i].len();
        if n == 0 {
            continue;
        }
        for j in 0..n {
            current_char = contents[i][j];

            if current_char.is_digit(10) {
                current_digits.push(current_char);

                adjacent_indices = generate_adjacent_indices(i, j, m, n);
                current_char_has_adjacent_symbols = adjacent_indices
                    .into_iter()
                    .map(|x| contents[x.0][x.1])
                    .any(|x| x != '.' && !x.is_digit(10));
                if current_char_has_adjacent_symbols {
                    current_digits_have_adjacent_symbols = true;
                }
            }
            if (j == n - 1 || !current_char.is_digit(10)) && current_digits_have_adjacent_symbols {
                engine_part_numbers.push(
                    current_digits
                        .iter()
                        .collect::<String>()
                        .parse::<u32>()
                        .ok()
                        .expect("current digits should only contain digits"),
                );
            }
            if !current_char.is_digit(10) {
                current_digits.clear();
                current_digits_have_adjacent_symbols = false;
            }
        }
    }
    return engine_part_numbers;
}

fn generate_adjacent_indices(i: usize, j: usize, m: usize, n: usize) -> Vec<(usize, usize)> {
    let i = i as i32;
    let j = j as i32;
    let n = n as i32;
    let m = m as i32;

    let adjacent_indices = vec![
        (i - 1, j - 1),
        (i - 1, j),
        (i - 1, j + 1),
        (i, j - 1),
        (i, j + 1),
        (i + 1, j - 1),
        (i + 1, j),
        (i + 1, j + 1),
    ];

    return adjacent_indices
        .into_iter()
        .filter(|x| x.0 >= 0 && x.0 < m && x.1 >= 0 && x.1 < n)
        .map(|x| (x.0 as usize, x.1 as usize))
        .collect();
}

#[cfg(test)]
mod tests {
    use super::{find_engine_part_numbers, generate_adjacent_indices, process_text};

    #[test]
    fn finds_correct_sum() {
        let example_input = String::from(
            "467..114.. \
            \n...*...... \
            \n..35..633. \
            \n......#... \
            \n617*...... \
            \n.....+.58. \
            \n..592..... \
            \n......755* \
            \n...$.*..21 \
            \n.664.598..",
        );
        assert_eq!(process_text(&example_input), 4382);
    }

    #[test]
    fn finds_correct_engine_numbers() {
        let example_input = vec![
            vec![
                '.', '.', '.', '.', '.', '.', '*', '.', '.', '.', '.', '.', '.', '.', '.', '.',
                '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
                '.', '.', '.', '.', '.', '.', '4', '4', '9', '.', '.', '.', '.', '.', '.', '.',
                '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '8', '0', '2', '.', '.',
                '.', '.', '.', '.', '.', '.', '7', '6', '2', '.', '.', '.', '.', '.', '5', '2',
                '2', '.', '.', '1', '3', '9', '+', '.', '.', '1', '5', '4', '.', '.', '2', '6',
                '6', '$', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
                '.', '.', '.', '.', '.', '.', '.', '.', '4', '6', '.', '3', '2', '3', '.', '.',
                '.', '.', '.', '7', '1', '2', '*', '.', '.', '.', '.', '.',
            ],
            vec![
                '.', '.', '.', '.', '8', '2', '7', '.', '.', '.', '.', '.', '.', '.', '.', '.',
                '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
                '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '3', '5', '.', '.',
                '.', '.', '.', '.', '6', '0', '0', '*', '2', '9', '.', '.', '.', '.', '.', '7',
                '2', '4', '.', '.', '.', '.', '.', '.', '.', '.', '4', '8', '8', '.', '.', '.',
                '.', '.', '.', '.', '.', '.', '.', '.', '.', '*', '.', '.', '.', '.', '.', '.',
                '.', '.', '.', '.', '.', '.', '.', '.', '.', '1', '4', '9', '*', '2', '2', '7',
                '.', '.', '.', '.', '.', '.', '.', '.', '.', '*', '.', '.', '.', '.', '.', '.',
                '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
            ],
            vec![
                '.', '.', '.', '.', '.', '.', '.', '.', '@', '.', '.', '.', '.', '.', '.', '2',
                '7', '8', '*', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '@',
                '.', '.', '1', '3', '5', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '5',
                '9', '4', '.', '.', '.', '.', '.', '.', '.', '.', '.', '4', '7', '0', '*', '.',
                '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '*', '.', '.', '.', '.', '5',
                '4', '0', '*', '.', '.', '.', '.', '.', '.', '.', '4', '2', '.', '.', '.', '.',
                '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
                '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '7', '7', '5', '.', '5', '3',
                '6', '.', '.', '7', '9', '0', '.', '.', '.', '.', '.', '.',
            ],
            vec![
                '.', '.', '.', '.', '.', '9', '9', '0', '.', '.', '.', '.', '.', '.', '.', '.',
                '.', '.', '.', '7', '4', '2', '.', '.', '.', '.', '.', '.', '1', '8', '1', '.',
                '.', '.', '.', '*', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
                '.', '.', '*', '.', '.', '@', '5', '2', '1', '.', '.', '.', '.', '.', '.', '.',
                '.', '.', '.', '.', '.', '.', '.', '.', '.', '7', '4', '3', '.', '.', '.', '.',
                '.', '.', '.', '5', '5', '2', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
                '.', '.', '3', '1', '0', '.', '.', '.', '*', '.', '.', '.', '.', '+', '7', '3',
                '8', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
                '.', '.', '.', '.', '.', '.', '*', '.', '.', '.', '.', '.',
            ],
            vec![
                '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
                '.', '.', '.', '.', '.', '.', '.', '1', '2', '5', '.', '.', '.', '.', '.', '.',
                '.', '3', '1', '1', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
                '7', '1', '5', '.', '.', '.', '.', '.', '.', '.', '.', '2', '6', '3', '.', '.',
                '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
                '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
                '.', '.', '.', '.', '.', '.', '.', '.', '.', '1', '7', '3', '.', '.', '.', '.',
                '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '2',
                '3', '5', '.', '.', '.', '.', '2', '9', '1', '.', '.', '.',
            ],
        ];
        assert_eq!(
            find_engine_part_numbers(example_input),
            vec![
                139, 154, 266, 46, 712, 827, 600, 29, 724, 488, 149, 227, 278, 135, 594, 470, 540,
                42, 775, 790, 990, 742, 181, 521, 743, 552, 738, 311, 715, 173, 291
            ]
        );
    }

    #[test]
    fn generates_correct_adjacent_indices() {
        assert_eq!(
            generate_adjacent_indices(0, 1, 3, 2),
            vec![(0, 0), (1, 0), (1, 1)]
        );
    }
}
