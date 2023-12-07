use regex::Regex;

const MAX_RED: u8 = 12;
const MAX_GREEN: u8 = 13;
const MAX_BLUE: u8 = 14;

pub fn run(input: &str) -> Option<String> {
    let game_id_regex = Regex::new(r"Game (\d+)").unwrap();
    let red_regex = Regex::new(r"(\d+) red").unwrap();
    let green_regex = Regex::new(r"(\d+) green").unwrap();
    let blue_regex = Regex::new(r"(\d+) blue").unwrap();
    let mut total = 0;
    for game in input.lines() {
        if is_valid(game, &red_regex, 1, MAX_RED)
            && is_valid(game, &green_regex, 1, MAX_GREEN)
            && is_valid(game, &blue_regex, 1, MAX_BLUE)
        {
            match game_id_regex.captures(game)
                .and_then(|cap| cap[1].parse::<u32>().ok()) {
                    Some(game_id) => total += game_id,
                    None => panic!("INVALID INPUT WEEWOO")
                };
        }
    }

    return Some(total.to_string());
}

fn is_valid(game: &str, r: &Regex, group: usize, max: u8) -> bool {
    for capture in r.captures_iter(game) {
        match capture[group].parse::<u8>() {
            Ok(number) => if number > max {
                return false;
            },
            Err(_) => return false
        };
    }

    return true;
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
        assert_eq!(Some("8".to_string()), actual);
    }
}