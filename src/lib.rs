pub fn get_solution(day: u8, part: u8) -> Option<fn(&str) -> Option<String>> {
    match (day, part) {
        (1, 1) => Some(day1_1::run),
        (1, 2) => Some(day1_2::run),
        (2, 1) => Some(day2_1::run),
        _ => None,
    }
}

mod day1_1;
mod day1_2;
mod day2_1;
