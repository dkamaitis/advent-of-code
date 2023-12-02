use std::env;
use std::error::Error;
use std::fmt;
use std::fs;
mod process_input;

#[derive(Debug)]
struct ArgumentError {
    message: String,
}

impl fmt::Display for ArgumentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Argument error: {}", self.message)
    }
}

impl Error for ArgumentError {}

fn check_arguments() -> Result<String, Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    if args.len() != 2 {
        return Err(Box::new(ArgumentError {
            message: "Expected exactly one argument: input file path".to_string(),
        }));
    }
    Ok(args[1].clone())
}

fn read_file(file_path: &str) -> Result<String, std::io::Error> {
    return fs::read_to_string(file_path);
}

fn main() {
    println!("Some test text");
    if let Ok(file_path) = check_arguments() {
        if let Ok(file_text) = read_file(&file_path.to_string()) {
            println!("Got to read file");
            let first_and_last_digit_sum = process_input::process_text(&file_text.to_string());
            println!("{}", first_and_last_digit_sum);
        } else {
            println!("Error while reading the input file");
        }
    }
}
