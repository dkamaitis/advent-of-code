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
    println!("{}: {:?}", &game_id?, &flat_game_sets);
    let any_larger_than_possible = flat_game_sets
        .iter()
        .any(|x| &x.0.parse::<u32>().unwrap() > &possible_games.get(x.1).unwrap());
    println!("{}", &any_larger_than_possible);
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
    use super::process_text;

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
}
