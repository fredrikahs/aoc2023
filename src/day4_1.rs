use std::collections::HashSet;
pub fn run(input: &str) -> Option<String> {
    let total = input.lines()
        .map(|line| {
            let numbers: Vec<HashSet<&str>> = line.split(':')
                .nth(1)
                .unwrap()
                .split('|')
                .map(|numbers| numbers.split_whitespace().collect::<HashSet<&str>>())
                .collect();

            let count = numbers[0].intersection(&numbers[1]).count() as u32;
            if count == 0 {
                return 0;
            }

            return 2u32.pow(count - 1);
        })
        .sum::<u32>();

    return Some(total.to_string());
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    fn test_run() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        let actual = run(input);
        assert_eq!(Some("13".to_string()), actual);
    }
}