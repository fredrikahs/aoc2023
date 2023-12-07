use std::env;
use std::fs;
use aoc2023::get_solution;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: aoc <day> <part> <input_path>");
        eprintln!("Example: aoc 1 1 input/input_1_1.txt");
        return;
    }

    let day = match args[1].parse::<u8>() {
        Ok(number) => number,
        Err(_) => {
            eprintln!("Invalid day");
            return;
        }
    };

    let part = match args[2].parse::<u8>() {
        Ok(number) => number,
        Err(_) => {
            eprintln!("Invalid part");
            return;
        }
    };

    let input = match fs::read_to_string(&args[3]) {
        Ok(content) => content,
        Err(error) => {
            eprintln!("Error reading file: {}", error);
            return;
        },
    };

    let solution = match get_solution(day, part) {
        Some(sln) => sln,
        None => {
            eprintln!("Day {0} Part {1} is not implemented", day, part);
            return;
        }
    };

    let Some(result) = solution(&input) else {
        eprintln!("No result");
        return;
    };

    println!("The result was {}", result);
}
