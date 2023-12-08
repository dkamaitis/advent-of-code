use core::f32::NEG_INFINITY;
use std::collections::HashMap;

pub fn process_text(contents: &str) -> u32 {
    return contents
        .split("\n")
        .map(multiply_max_cubes)
        .filter_map(|x| x)
        .sum();
}

pub fn multiply_max_cubes(text: &str) -> Option<u32> {
    let mut max_cubes: HashMap<&str, u32> = HashMap::from([
        ("red", NEG_INFINITY as u32),
        ("green", NEG_INFINITY as u32),
        ("blue", NEG_INFINITY as u32),
    ]);

    let initial_split = text.split_once(':');

    let flat_game_sets = split_game_info(initial_split?.1);
    for cube_set_x in flat_game_sets {
        let cube_count = cube_set_x
            .0
            .parse::<u32>()
            .expect("first element within dict-like text must be a digit");
        if &cube_count
            > &max_cubes
                .get(cube_set_x.1)
                .expect("color in text must be red, green, or blue")
        {
            max_cubes
                .entry(cube_set_x.1)
                .and_modify(|e| (*e = cube_count))
                .or_insert(cube_count);
        }
    }

    return max_cubes.into_values().reduce(|acc, e| (acc * e));
}

pub fn split_game_info(game_info: &str) -> Vec<(&str, &str)> {
    let game_sets: Vec<Vec<(&str, &str)>> = game_info
        .split(';')
        .map(|cube_set| {
            cube_set
                .trim()
                .split(',')
                .filter_map(|single_color_count| single_color_count.trim().split_once(' '))
                .collect()
        })
        .collect();
    return game_sets
        .iter()
        .flat_map(|inner_vec| inner_vec.iter())
        .cloned()
        .collect();
}

#[cfg(test)]
mod tests {
    use super::{multiply_max_cubes, process_text, split_game_info};

    #[test]
    fn finds_correct_sum() {
        let example_input = String::from(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green \
            \nGame 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue \
            \nGame 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red \
            \nGame 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red \
            \nGame 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        );
        assert_eq!(process_text(&example_input), 2286);
    }

    #[test]
    fn multiplies_cubes_correctly() {
        let test_cases: Vec<(&str, u32)> = vec![
            ("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", 48),
            (
                "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
                12,
            ),
            (
                "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
                1560,
            ),
            (
                "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
                630,
            ),
            ("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green", 36),
        ];
        for (input, expected) in test_cases {
            assert_eq!(
                multiply_max_cubes(input).expect("should return a value for each test case"),
                expected
            );
        }
    }

    #[test]
    fn splits_game_info_correctly() {
        let test_cases: Vec<(&str, Vec<(&str, &str)>)> = vec![
            (
                "1 red, 4 green; 6 red, 1 green; 10 red; 1 blue, 2 green; 4 green, 3 red; \
                 1 blue, 8 red",
                vec![
                    ("1", "red"),
                    ("4", "green"),
                    ("6", "red"),
                    ("1", "green"),
                    ("10", "red"),
                    ("1", "blue"),
                    ("2", "green"),
                    ("4", "green"),
                    ("3", "red"),
                    ("1", "blue"),
                    ("8", "red"),
                ],
            ),
            (
                "9 blue, 13 green, 1 red; 10 green, 4 blue, 4 red; 3 red, 4 blue, 14 green; \
                 13 blue, 1 red, 12 green",
                vec![
                    ("9", "blue"),
                    ("13", "green"),
                    ("1", "red"),
                    ("10", "green"),
                    ("4", "blue"),
                    ("4", "red"),
                    ("3", "red"),
                    ("4", "blue"),
                    ("14", "green"),
                    ("13", "blue"),
                    ("1", "red"),
                    ("12", "green"),
                ],
            ),
        ];
        for (input, expected) in test_cases {
            assert_eq!(split_game_info(input), expected);
        }
    }
}
