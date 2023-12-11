use std::env;
use std::fs;
mod process_input;

fn read_file(file_path: &str) -> Result<String, std::io::Error> {
    return fs::read_to_string(file_path);
}

fn main() {
    let args: Vec<_> = env::args().collect();
    let file_path = match args.len() {
        2 => &args[1],
        _ => panic!("Expected exactly one argument: input file path"),
    };
    let file_text = read_file(file_path).expect("Input file should be available in this project");
    let sum_2 = process_input::process_text(&file_text);
    println!("{}", sum_2);
}
