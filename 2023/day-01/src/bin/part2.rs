use regex::Regex;
use std::str::FromStr;

fn main() {
    let input = include_str!("input1.txt");
    let calibration = parse_calibration(input);
    println!("Calibration: {:?}", calibration);
}

#[derive(Debug, PartialEq)]
struct Calibration(u32);

#[derive(Debug, PartialEq)]
enum Token {
    NumberDigit(u32),
    NumberWord(u32),
}

impl FromStr for Token {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<u32>() {
            Ok(n) => Ok(Token::NumberDigit(n)),
            Err(_) => match s {
                "one" => Ok(Token::NumberWord(1u32)),
                "two" => Ok(Token::NumberWord(2u32)),
                "three" => Ok(Token::NumberWord(3u32)),
                "four" => Ok(Token::NumberWord(4u32)),
                "five" => Ok(Token::NumberWord(5u32)),
                "six" => Ok(Token::NumberWord(6u32)),
                "seven" => Ok(Token::NumberWord(7u32)),
                "eight" => Ok(Token::NumberWord(8u32)),
                "nine" => Ok(Token::NumberWord(9u32)),
                _ => Err(format!("Invalid token: {}", s)),
            },
        }
    }
}

impl Clone for Token {
    fn clone(&self) -> Self {
        match self {
            Token::NumberDigit(n) => Token::NumberDigit(*n),
            Token::NumberWord(n) => Token::NumberWord(*n),
        }
    }
}

impl Token {
    fn as_u32(&self) -> u32 {
        match self {
            Token::NumberDigit(n) => *n,
            Token::NumberWord(n) => *n,
        }
    }
}

fn parse_calibration(input: &str) -> Result<Calibration, String> {
    let line_values: Result<Vec<u32>, String> = input
        .lines()
        .into_iter()
        .map(parse_tokens)
        .map(|line_as_tokens| line_as_tokens.map(first_and_last))
        .collect();

    line_values.map(|line_values| Calibration(line_values.iter().sum()))
}

fn first_and_last(tokens: Vec<Token>) -> u32 {
    match tokens.len() {
        0 => 0,
        _ => tokens[0].as_u32() * 10 + tokens[tokens.len() - 1].as_u32(),
    }
}

fn parse_tokens(input: &str) -> Result<Vec<Token>, String> {
    let re = Regex::new(r"([123456789]|one|two|three|four|five|six|seven|eight|nine)").unwrap();

    re.captures_iter(input)
        .map(|capture| match capture.get(0) {
            Some(capture) => capture.as_str().parse::<Token>(),
            None => Err(format!("Invalid capture: {:?}", capture)),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_tokens() {
        let input = "9vxfg";
        let expected = vec![Token::NumberDigit(9)];
        assert_eq!(parse_tokens(input), Ok(expected));
    }

    #[test]
    fn test_parse_calibration() {
        let input = r#"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"#;
        let calibration = parse_calibration(input);
        assert_eq!(calibration, Ok(Calibration(281)));
    }

    #[test]
    fn test_parse_calibration_empty_input() {
        let input = "";
        let calibration = parse_calibration(input);
        assert_eq!(calibration, Ok(Calibration(0)));
    }

    #[test]
    fn test_parse_calibration_invalid_token() {
        let input = r#"one two three
four five invalid
seven eight nine"#;
        let calibration = parse_calibration(input);
        assert_eq!(calibration, Ok(Calibration(13 + 45 + 79)));
    }
}
