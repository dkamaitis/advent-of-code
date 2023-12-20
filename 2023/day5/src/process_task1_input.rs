use std::collections::HashMap;

pub fn process_text(contents: &str) -> u64 {
    let mut keys: Vec<&str> = vec![];
    let mappings: HashMap<&str, Vec<Vec<u64>>> = contents
        .split("\n\n")
        .filter_map(|mapping| {
            let mut split = mapping.split(':');
            match (split.next(), split.next()) {
                (Some(key), Some(value)) => {
                    keys.push(key);
                    Some((key, split_entries(value)))
                }
                _ => None,
            }
        })
        .collect();

    let mut source_ids: Vec<u64> = mappings
        .get(keys[0])
        .expect("ordered keys in vector should match hashmap keys")[0]
        .clone();
    for destination_map_name in keys[1..].iter() {
        source_ids = map_source_to_destination(
            &source_ids,
            mappings
                .get(destination_map_name)
                .expect("ordered keys in vector should match hashmap keys"),
        );
    }
    return *source_ids
        .iter()
        .min()
        .expect("seeds should eventually translate into location IDs");
}

fn map_source_to_destination(source_ids: &Vec<u64>, destination_maps: &Vec<Vec<u64>>) -> Vec<u64> {
    return source_ids
        .iter()
        .map(|source_id| {
            let matching_map: Option<&Vec<u64>> = destination_maps
                .iter()
                .filter(|map_x| *source_id >= map_x[1] && *source_id <= (map_x[1] + map_x[2]))
                .next();
            return match matching_map {
                Some(mapping) => mapping[0] + (*source_id - mapping[1]),
                None => *source_id,
            };
        })
        .collect();
}

fn split_entries(entries: &str) -> Vec<Vec<u64>> {
    return entries
        .trim()
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|number| {
                    number
                        .parse::<u64>()
                        .expect("each line should only contain a list of space separated numbers")
                })
                .collect()
        })
        .collect();
}

#[cfg(test)]
mod tests {
    use super::{map_source_to_destination, process_text, split_entries};

    #[test]
    fn finds_correct_solution() {
        let example_input = String::from(
            "seeds: 79 14 55 13 \
            \n\
            \nseed-to-soil map: \
            \n50 98 2 \
            \n52 50 48 \
            \n\
            \nsoil-to-fertilizer map: \
            \n0 15 37 \
            \n37 52 2 \
            \n39 0 15 \
            \n\
            \nfertilizer-to-water map: \
            \n49 53 8 \
            \n0 11 42 \
            \n42 0 7 \
            \n57 7 4 \
            \n\
            \nwater-to-light map: \
            \n88 18 7 \
            \n18 25 70 \
            \n\
            \nlight-to-temperature map: \
            \n45 77 23 \
            \n81 45 19 \
            \n68 64 13 \
            \n\
            \ntemperature-to-humidity map: \
            \n0 69 1 \
            \n1 0 69 \
            \n\
            \nhumidity-to-location map: \
            \n60 56 37 \
            \n56 93 4",
        );
        assert_eq!(process_text(&example_input), 35);
    }

    #[test]
    fn maps_source_to_destination_correctly() {
        let test_cases = vec![
            (
                vec![79, 14, 55, 13],
                vec![vec![50, 98, 2], vec![52, 50, 48]],
                vec![81, 14, 57, 13],
            ),
            (
                vec![81, 14, 57, 13],
                vec![vec![0, 15, 37], vec![37, 52, 2], vec![39, 0, 15]],
                vec![81, 53, 57, 52],
            ),
        ];
        for (input1, input2, expected) in test_cases {
            assert_eq!(map_source_to_destination(&input1, &input2), expected);
        }
    }

    #[test]
    fn splits_mapping_entries_correctly() {
        let example_input = String::from(
            "49 53 8 \
            \n0 11 42 \
            \n42 0 7 \
            \n57 7 4",
        );
        assert_eq!(
            split_entries(&example_input),
            vec![
                vec![49, 53, 8],
                vec![0, 11, 42],
                vec![42, 0, 7],
                vec![57, 7, 4]
            ]
        );
    }
}
