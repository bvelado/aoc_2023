use regex::{Match, Regex};

pub fn part1(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            let mut result: Option<i32> = None;
            let first_digit = line
                .chars()
                .find_map(|c| if c.is_numeric() { Some(c) } else { None });
            let last_digit =
                line.chars()
                    .rev()
                    .find_map(|c| if c.is_numeric() { Some(c) } else { None });
            if let (Some(fd), Some(ld)) = (first_digit, last_digit) {
                result = format!("{}{}", fd, ld).parse().ok();
            }
            result
        })
        .fold(
            0i32,
            |acc: i32, calibration_value| match calibration_value {
                Some(cv) => acc + cv,
                None => acc,
            },
        )
}

pub fn part2(input: &str) -> i32 {
    let re = Regex::new(r"(\d|one|two|three|four|five|six|seven|eight|nine)").unwrap();
    input
        .lines()
        .map(|line| {
            if let Some(digits) = find_digits(line, &re) {
                let number = format!("{}{}", map_digit(digits.0), map_digit(digits.1));
                return number.parse().unwrap();
            }
            0i32
        })
        .sum()
}

fn find_digits(input: &str, regex: &Regex) -> Option<(String, String)> {
    let matches: Vec<Match> = regex.find_iter(&input).collect();
    if matches.len() < 2 {
        return None;
    }
    Some((
        matches[0].as_str().to_string(),
        matches.last().unwrap().as_str().to_string(),
    ))
}

fn map_digit(s: String) -> i32 {
    match s.parse().ok() {
        Some(i) => i,
        None => match s.as_str() {
            "one" => 1,
            "two" => 2,
            "three" => 3,
            "four" => 4,
            "five" => 5,
            "six" => 6,
            "seven" => 7,
            "eight" => 8,
            "nine" => 9,
            _ => 0,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_works() {
        let input = include_str!("../input1_test.txt");
        let result = part1(input);
        assert_eq!(result, 142);
    }
    #[test]
    fn regex_works() {
        let re = Regex::new(r"(\d|one|two|three|four|five|six|seven|eight|nine)");
        assert!(re.is_ok());
    }

    #[test]
    fn part_2_works() {
        let input = include_str!("../input2_test.txt");
        let result = part2(input);
        assert_eq!(result, 281);
    }
}
