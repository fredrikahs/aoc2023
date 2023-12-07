const DIGITS: &[(&str, u32)] = &[
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
    ("1", 1),
    ("2", 2),
    ("3", 3),
    ("4", 4),
    ("5", 5),
    ("6", 6),
    ("7", 7),
    ("8", 8),
    ("9", 9)
];

pub fn run(input: &str) -> Option<String> {
    return Some(input
        .lines()
        .filter_map(|s| get_both(s))
        .sum::<u32>()
        .to_string());
}

pub fn get_both(s: &str) -> Option<u32> {
    let first = get_first(s);
    let last = get_last(s);
    if first.is_none() {
        return None;
    }

    return Some(first.unwrap() * 10 + last.unwrap());
}

fn get_first(s: &str) -> Option<u32> {
    for i in 0..s.len() {
        for &(substr, character) in DIGITS {
            if s[i..].starts_with(substr) {
                return Some(character);
            }
        }
    }

    return None;
}

fn get_last(s: &str) -> Option<u32> {
    for i in (0..s.len()).rev() {
        for &(substr, digit) in DIGITS {
            if s[i..].starts_with(substr) {
                return Some(digit);
            }
        }
    }

    return None;
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("two1nine", 2)]
    #[case("eightwothree", 8)]
    #[case("abcone2threexyz", 1)]
    #[case("xtwone3four", 2)]
    #[case("4nineeightseven2", 4)]
    #[case("zoneight234", 1)]
    #[case("7pqrstsixteen", 7)]
    fn test_first(#[case] input: &str, #[case] expected: u32) {
        let actual = get_first(input);
        if let Some(d) = actual {
            assert_eq!(d, expected);
        }
    }

    #[rstest]
    #[case("two1nine", 29)]
    #[case("two", 22)]
    #[case("eightwothree", 83)]
    #[case("abcone2threexyz", 13)]
    #[case("xtwone3four", 24)]
    #[case("4nineeightseven2", 42)]
    #[case("zoneight234", 14)]
    #[case("7pqrstsixteen", 76)]
    fn test_both(#[case] input: &str, #[case] expected: u32) {
        let actual = get_both(input);
        if let Some(d) = actual {
            assert_eq!(d, expected);
        }
    }

    #[rstest]
    #[case("two1nine", 9)]
    #[case("9", 9)]
    #[case("eightwothree", 3)]
    #[case("abcone2threexyz", 3)]
    #[case("xtwone3four", 4)]
    #[case("4nineeightseven2", 2)]
    #[case("zoneight234", 4)]
    #[case("7pqrstsixteen", 6)]
    fn test_last(#[case] input: &str, #[case] expected: u32) {
        let actual = get_last(input);
        if let Some(d) = actual {
            assert_eq!(d, expected);
        }
    }

    #[rstest]
    fn test_run() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

        let actual = run(input);
        assert_eq!(Some("281".to_string()), actual);
    }
}