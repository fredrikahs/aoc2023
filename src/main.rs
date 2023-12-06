use std::env;
use std::fs;
use aoc2023::{
    day1_1,
    day1_2,
};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: aoc <day> <part> <input_path>");
        eprintln!("Example: aoc 1 1 input/input_1_1.txt");
        return;
    }

    let input = match fs::read_to_string(&args[3]) {
        Ok(content) => content,
        Err(error) => {
            eprintln!("Error reading file: {}", error);
            return;
        },
    };

    let total = match (args[1].as_str(), args[2].as_str()) {
        ("1", "1") => day1_1::run(&input),
        ("1", "2") => day1_2::run(&input),
        _ => {
            eprintln!("Unknown day or part: {} {}", args[1], args[2]);
            return;
        },
    };

    println!("The total was {}", total);
}
