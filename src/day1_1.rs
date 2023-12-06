pub fn run(input: &str) -> u32 {
    return input
        .lines()
        .map(|s| get_both(s))
        .sum();
}

fn get_both(x: &str) -> u32 {
    let first = get_digit(x, false);
    let last = get_digit(x, true);

    return first * 10 + last;
}

fn get_digit(x: &str, rev: bool) -> u32 {
    for i in 0..x.len() {
        let idx = if rev {
            x.len() - 1 - i
        } else {
            i
        };
        let ch = x.as_bytes()[idx];
        if ch.is_ascii_digit() {
            return ch as u32 - '0' as u32;
        }
    }

    panic!("INVALID INPUT WEEWOO");
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("2", 22)]
    #[case("a1", 11)]
    #[case("2a", 22)]
    #[case("1abc2", 12)]
    #[case("ab1c2", 12)]
    #[case("a1c", 11)]
    #[case("pqr3stu8vwx", 38)]
    #[case("a1b2c3d4e5f", 15)]
    #[case("treb7uchet", 77)]
    fn test_both(#[case] input: &str, #[case] expected: u32) {
        let actual = get_both(input);
        assert_eq!(actual, expected);
    }

    #[rstest]
    #[case("2", 2)]
    #[case("a1", 1)]
    #[case("2a", 2)]
    #[case("1abc2", 1)]
    #[case("pqr3stu8vwx", 3)]
    #[case("a1b2c3d4e5f", 1)]
    #[case("treb7uchet", 7)]
    fn test_first_digit(#[case] input: &str, #[case] expected: u32) {
        let actual = get_digit(input, false);
        assert_eq!(actual, expected);
    }

    #[rstest]
    #[case("2", 2)]
    #[case("a1", 1)]
    #[case("2a", 2)]
    #[case("1abc2", 2)]
    #[case("pqr3stu8vwx", 8)]
    #[case("a1b2c3d4e5f", 5)]
    #[case("treb7uchet", 7)]
    fn test_last_digit(#[case] input: &str, #[case] expected: u32) {
        let actual = get_digit(input, true);
        assert_eq!(actual, expected);
    }
}
