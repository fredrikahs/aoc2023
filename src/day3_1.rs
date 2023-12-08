use regex::Regex;

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Position {
    pub line: usize,
    pub start: usize,
    pub end: usize,
    pub number: u32,
}

pub fn run(input: &str) -> Option<String> {
    let symbol_positions = get_symbol_positions(&input);
    let number_positions = get_numbers_with_positions(&input);

    let total = number_positions
        .iter()
        .filter_map(move |pos| {
            let first_line = pos.line.saturating_sub(1);
            let first_char = pos.start.saturating_sub(1);

            for v in first_line..=(pos.line+1) {
                for h in first_char..=(pos.end+1) {
                    if symbol_positions.contains(&(v,h)) {
                        return Some(pos.number)
                    }
                }
            }

            None
        })
        .sum::<u32>();

    return Some(total.to_string());
}

pub fn get_numbers_with_positions(input: &str) -> Vec<Position> {
    let number_regex = Regex::new(r"(\d+)").unwrap();
    return input
        .lines()
        .enumerate()
        .flat_map(|(line_idx, line)| {
            number_regex.find_iter(line)
                .filter_map(move |m|
                    m.as_str()
                        .parse::<u32>()
                        .ok()
                        .map(|number|
                            Position {
                                line: line_idx,
                                start: m.start(),
                                end: m.end() - 1,
                                number
                            }
                        ))
        })
        .collect();
}

fn get_symbol_positions(input: &str) -> Vec<(usize, usize)> {
    return input.lines()
        .enumerate()
        .flat_map(|(line_idx, line)| {
            line.char_indices()
                .filter_map(move |(char_idx, char)| {
                    if char == '.' || char.is_digit(10) {
                        return None;
                    }

                    return Some((line_idx, char_idx));
                })
        }).collect();
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    fn test_run() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        let actual = run(input);
        assert_eq!(Some("4361".to_string()), actual);
    }

    #[rstest]
    fn test_symbol_positions() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        let actual = get_symbol_positions(input);
        let expected = [(1, 3), (3, 6), (4, 3), (5, 5), (8, 3), (8, 5)].to_vec();
        assert_eq!(expected, actual);
    }

    #[rstest]
    fn test_number_positions() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        let expected = [
            (0, 0, 2, 467),
            (0, 5, 7, 114),
            (2, 2, 3, 35),
            (2, 6, 8, 633),
            (4, 0, 2, 617),
            (5, 7, 8, 58),
            (6, 2, 4, 592),
            (7, 6, 8, 755),
            (9, 1, 3, 664),
            (9, 5, 7, 598)
        ].map(|t| Position {
            line: t.0,
            start: t.1,
            end: t.2,
            number: t.3
        }).to_vec();

        let actual = get_numbers_with_positions(input);

        assert_eq!(expected, actual);
    }
}