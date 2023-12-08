use std::collections::HashMap;

pub fn process_text(contents: &str) -> u32 {
    return contents
        .split("\n")
        .map(find_possible_ids)
        .filter_map(|x| x)
        .sum();
}

pub fn find_possible_ids(text: &str) -> Option<u32> {
    let possible_games: HashMap<&str, u32> =
        HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);

    let initial_split = text.split_once(':');
    let game_id: Option<u32> = initial_split?.0.replace("Game ", "").parse::<u32>().ok();

    let flat_game_sets = split_game_info(initial_split?.1);

    let any_larger_than_possible = flat_game_sets.iter().any(|x| {
        &x.0.parse::<u32>()
            .expect("first element within dict-like text must be a digit")
            > &possible_games
                .get(x.1)
                .expect("color in text must be red, green, or blue")
    });

    if !any_larger_than_possible {
        return game_id;
    } else {
        return None;
    }
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
    use super::{find_possible_ids, process_text, split_game_info};

    #[test]
    fn finds_correct_sum() {
        let example_input = String::from(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green \
            \nGame 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue \
            \nGame 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red \
            \nGame 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red \
            \nGame 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        );
        assert_eq!(process_text(&example_input), 8);
    }

    #[test]
    fn finds_correct_ids() {
        let test_cases: Vec<(&str, Option<u32>)> = vec![
            (
                "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
                Some(1),
            ),
            (
                "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
                Some(2),
            ),
            (
                "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
                None,
            ),
            (
                "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
                None,
            ),
            (
                "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
                Some(5),
            ),
        ];
        for (input, expected) in test_cases {
            assert_eq!(find_possible_ids(input), expected);
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
