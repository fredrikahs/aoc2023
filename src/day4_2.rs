use std::collections::HashSet;

#[derive(Clone, Copy)]
struct Card {
    number: usize,
    wins: usize,
    card_count: usize
}

pub fn run(input: &str) -> Option<String> {
    let mut cards = input.lines()
        .enumerate()
        .map(|(line_idx, line)| {
            let numbers: Vec<HashSet<&str>> = line.split(':')
                .nth(1)
                .unwrap()
                .split('|')
                .map(|numbers| numbers.split_whitespace().collect::<HashSet<&str>>())
                .collect();

            let count = numbers[0].intersection(&numbers[1]).count();

            Card {
                number: line_idx,
                wins: count,
                card_count: 1
            }
        })
        .collect::<Vec<Card>>();

    for i in 0..cards.len() {
        let wins = cards[i].wins;
        let card_count = cards[i].card_count;

        for w in 1..=wins {
            let next_index = i + w;
            if next_index < cards.len() {
                cards[next_index].card_count += card_count;
            }
        }
    }

    let total = cards
        .iter()
        .map(|c| c.card_count)
        .sum::<usize>();

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
        assert_eq!(Some("30".to_string()), actual);
    }
}