use std::collections::HashMap;

pub fn process_text(contents: &str) -> u32 {
    let new_text: HashMap<&str, Vec<Vec<u32>>> = contents
        .split("\n\n")
        .filter_map(|mapping| {
            let mut split = mapping.split(':');
            match (split.next(), split.next()) {
                (Some(key), Some(value)) => Some((key, split_entries(value))),
                _ => None,
            }
        })
        .collect();
    println!("{:?}", new_text);
    return 42;
}

fn split_entries(entries: &str) -> Vec<Vec<u32>> {
    return entries
        .trim()
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|number| {
                    number
                        .parse::<u32>()
                        .expect("each line should only contain a list of space separated numbers")
                })
                .collect()
        })
        .collect();
}
