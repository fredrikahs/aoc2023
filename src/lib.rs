pub fn get_solution(day: u8, part: u8) -> Option<fn(&str) -> Option<String>> {
    match (day, part) {
        (1, 1) => Some(day1_1::run),
        (1, 2) => Some(day1_2::run),
        (2, 1) => Some(day2_1::run),
        (2, 2) => Some(day2_2::run),
        (3, 1) => Some(day3_1::run),
        (3, 2) => Some(day3_2::run),
        (4, 1) => Some(day4_1::run),
        (4, 2) => Some(day4_2::run),
        // (5, 1) => Some(day5_1::run),
        // (5, 2) => Some(day5_2::run),
        // (6, 1) => Some(day6_1::run),
        // (6, 2) => Some(day6_2::run),
        // (7, 1) => Some(day7_1::run),
        // (7, 2) => Some(day7_2::run),
        // (8, 1) => Some(day8_1::run),
        // (8, 2) => Some(day8_2::run),
        // (9, 1) => Some(day9_1::run),
        // (9, 2) => Some(day9_2::run),
        // (10, 1) => Some(day10_1::run),
        // (10, 2) => Some(day10_2::run),
        // (11, 1) => Some(day11_1::run),
        // (11, 2) => Some(day11_2::run),
        // (12, 1) => Some(day12_1::run),
        // (12, 2) => Some(day12_2::run),
        // (13, 1) => Some(day13_1::run),
        // (13, 2) => Some(day13_2::run),
        // (14, 1) => Some(day14_1::run),
        // (14, 2) => Some(day14_2::run),
        // (15, 1) => Some(day15_1::run),
        // (15, 2) => Some(day15_2::run),
        // (16, 1) => Some(day16_1::run),
        // (16, 2) => Some(day16_2::run),
        // (17, 1) => Some(day17_1::run),
        // (17, 2) => Some(day17_2::run),
        // (18, 1) => Some(day18_1::run),
        // (18, 2) => Some(day18_2::run),
        // (19, 1) => Some(day19_1::run),
        // (19, 2) => Some(day19_2::run),
        // (20, 1) => Some(day20_1::run),
        // (20, 2) => Some(day20_2::run),
        // (21, 1) => Some(day21_1::run),
        // (21, 2) => Some(day21_2::run),
        // (22, 1) => Some(day22_1::run),
        // (22, 2) => Some(day22_2::run),
        // (23, 1) => Some(day23_1::run),
        // (23, 2) => Some(day23_2::run),
        // (24, 1) => Some(day24_1::run),
        // (24, 2) => Some(day24_2::run),
        // (25, 1) => Some(day25_1::run),
        // (25, 2) => Some(day25_2::run),
        _ => None,
    }
}

mod day1_1;
mod day1_2;
mod day2_1;
mod day2_2;
mod day3_1;
mod day3_2;
mod day4_1;
mod day4_2;
// mod day5_1;
// mod day5_2;
// mod day6_1;
// mod day6_2;
// mod day7_1;
// mod day7_2;
// mod day8_1;
// mod day8_2;
// mod day9_1;
// mod day9_2;
// mod day10_1;
// mod day10_2;
// mod day11_1;
// mod day11_2;
// mod day12_1;
// mod day12_2;
// mod day13_1;
// mod day13_2;
// mod day14_1;
// mod day14_2;
// mod day15_1;
// mod day15_2;
// mod day16_1;
// mod day16_2;
// mod day17_1;
// mod day17_2;
// mod day18_1;
// mod day18_2;
// mod day19_1;
// mod day19_2;
// mod day20_1;
// mod day20_2;
// mod day21_1;
// mod day21_2;
// mod day22_1;
// mod day22_2;
// mod day23_1;
// mod day23_2;
// mod day24_1;
// mod day24_2;
// mod day25_1;
// mod day25_2;