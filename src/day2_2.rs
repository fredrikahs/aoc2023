use regex::Regex;

pub fn run(input: &str) -> Option<String> {
    let red_regex = Regex::new(r"(\d+) red").unwrap();
    let green_regex = Regex::new(r"(\d+) green").unwrap();
    let blue_regex = Regex::new(r"(\d+) blue").unwrap();

    let total = input
        .lines()
        .map(|game| {
            get_max(game, &red_regex, 1)
            * get_max(game, &green_regex, 1)
            * get_max(game, &blue_regex, 1)
        })
        .sum::<u32>();

    return Some(total.to_string());
}

fn get_max(game: &str, r: &Regex, group: usize) -> u32 {
    return r.captures_iter(game)
        .filter_map(|cap| cap[group].parse::<u32>().ok())
        .max()
        .unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    fn test_run() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        let actual = run(input);
        assert_eq!(Some("2286".to_string()), actual);
    }
}