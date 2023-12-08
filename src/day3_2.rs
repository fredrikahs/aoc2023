use crate::day3_1::{
    Position,
    get_numbers_with_positions
};

struct Symbol {
    adjacent_positions: Vec<Position>,
    line: usize,
    column: usize
}

pub fn run(input: &str) -> Option<String> {
    let mut symbol_positions = get_symbol_positions(&input);
    let number_positions = get_numbers_with_positions(&input);

    'nums: for pos in number_positions {
        for line in pos.line.saturating_sub(1)..=(pos.line + 1) {
            for col in pos.start.saturating_sub(1)..=(pos.end + 1) {
                if let Some(symbol) = symbol_positions.iter_mut()
                    .find(|s| s.line == line && s.column == col) {
                    symbol.adjacent_positions.push(pos);
                    continue 'nums;
                }
            }
        }
    }

    let total = symbol_positions
        .iter()
        .filter_map(|s| {
            if s.adjacent_positions.len() == 2 {
                return Some(s.adjacent_positions.iter()
                    .fold(1, |acc, pos| pos.number * acc))
            }

            None
        })
        .sum::<u32>();

    return Some(total.to_string());
}

fn get_symbol_positions(input: &str) -> Vec<Symbol> {
    return input.lines()
        .enumerate()
        .flat_map(|(line_idx, line)| {
            line.char_indices()
                .filter_map(move |(column, char)| {
                    if char == '*' {
                        return Some(Symbol {
                            line: line_idx,
                            column,
                            adjacent_positions: Vec::new()
                        });
                    }

                    return None;
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
        assert_eq!(Some("467835".to_string()), actual);
    }
}