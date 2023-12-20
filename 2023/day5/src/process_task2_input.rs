use std::{
    cmp::{max, min},
    collections::HashMap,
};

pub fn process_text(contents: &str) -> u64 {
    let mut mappings_lines = contents.split("\n\n");

    let seed_ranges: Vec<(u64, u64)> = split_seeds(
        mappings_lines
            .next()
            .expect("seeds should be in the first line of input followed by blank line"),
    );

    let mut mappings_ordered_keys: Vec<&str> = vec![];
    let mappings: HashMap<&str, Vec<(u64, u64, u64)>> = mappings_lines
        .filter_map(|mapping| {
            let mut split = mapping.split(':');
            match (split.next(), split.next()) {
                (Some(key), Some(value)) => {
                    mappings_ordered_keys.push(key);
                    Some((key, split_mapping_entries(value)))
                }
                _ => None,
            }
        })
        .collect();

    let mut source_ranges: Vec<(u64, u64)> = seed_ranges;
    for destination_map_name in mappings_ordered_keys.iter() {
        source_ranges = source_ranges
            .into_iter()
            .map(|source_range| {
                map_ranges_to_destination(
                    source_range,
                    mappings
                        .get(destination_map_name)
                        .expect("ordered keys in vector should match hashmap keys")
                        .clone(),
                )
            })
            .flatten()
            .collect();
    }
    return source_ranges
        .into_iter()
        .map(|(range_start, _)| range_start)
        .min()
        .expect("seeds should eventually translate into location IDs");
}

fn map_ranges_to_destination(
    input_range: (u64, u64),
    destination_maps: Vec<(u64, u64, u64)>,
) -> Vec<(u64, u64)> {
    let mut input_ranges: Vec<(u64, u64)> = vec![(input_range.0, input_range.0 + input_range.1)];
    let mut destination_ranges: Vec<(u64, u64)> = vec![];
    for (destionation_start, source_start, source_length) in destination_maps {
        let source_end = source_start + source_length;
        let mut non_matching_ranges: Vec<(u64, u64)> = vec![];
        for (input_start, input_end) in input_ranges {
            // Before matching range
            if input_start < source_start {
                non_matching_ranges.push((input_start, min(source_start, input_end)));
            }

            // After matching range
            if input_end > source_end {
                non_matching_ranges.push((max(source_end, input_start), input_end));
            }

            // Matching range
            if input_start < source_end && input_end > source_start {
                let matching_range = (max(source_start, input_start), min(source_end, input_end));
                destination_ranges.push((
                    destionation_start + (matching_range.0 - source_start),
                    destionation_start + (matching_range.1 - source_start),
                ));
            }
        }
        input_ranges = non_matching_ranges;
    }
    // Input ranges at this point only contains non-matching input ranges
    let result = [destination_ranges, input_ranges]
        .concat()
        .into_iter()
        .map(|(range_start, range_end)| (range_start, range_end - range_start))
        .collect();
    return result;
}

fn split_seeds(seeds_line: &str) -> Vec<(u64, u64)> {
    return seeds_line
        .split_once(':')
        .expect("seeds_line should have key-value structure separated by ':'")
        .1
        .split_whitespace()
        .map(|number| {
            number
                .parse::<u64>()
                .expect("each line should only contain a list of space separated numbers")
        })
        .collect::<Vec<u64>>()
        .chunks_exact(2)
        .map(|chunk| (chunk[0], chunk[1]))
        .collect();
}

fn split_mapping_entries(entries: &str) -> Vec<(u64, u64, u64)> {
    return entries
        .trim()
        .lines()
        .map(|line| {
            let mut numbers = line.split_whitespace().map(|number| {
                number
                    .parse::<u64>()
                    .expect("each line should only contain a list of space separated numbers")
            });
            (
                numbers
                    .next()
                    .expect("map entries should contain three numbers per line"),
                numbers
                    .next()
                    .expect("map entries should contain three numbers per line"),
                numbers
                    .next()
                    .expect("map entries should contain three numbers per line"),
            )
        })
        .collect();
}

#[cfg(test)]
mod tests {
    use super::{map_ranges_to_destination, process_text, split_mapping_entries, split_seeds};

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
        assert_eq!(process_text(&example_input), 46);
    }

    #[test]
    fn maps_ranges_to_destination_correctly() {
        let test_cases = vec![
            ((79, 14), vec![(50, 98, 2), (52, 50, 48)], vec![(81, 14)]),
            ((55, 13), vec![(50, 98, 2), (52, 50, 48)], vec![(57, 13)]),
            (
                (81, 14),
                vec![(0, 15, 37), (37, 52, 2), (39, 0, 15)],
                vec![(81, 14)],
            ),
            (
                (57, 13),
                vec![(0, 15, 37), (37, 52, 2), (39, 0, 15)],
                vec![(57, 13)],
            ),
        ];
        for (input1, input2, expected) in test_cases {
            assert_eq!(map_ranges_to_destination(input1, input2), expected);
        }
    }

    #[test]
    fn splits_seeds_correctly() {
        let example_input = String::from("seeds: 79 14 55 13");
        assert_eq!(split_seeds(&example_input), vec![(79, 14), (55, 13)]);
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
            split_mapping_entries(&example_input),
            vec![(49, 53, 8), (0, 11, 42), (42, 0, 7), (57, 7, 4)]
        );
    }
}
