use regex::Regex;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Line {
    length: usize,
    numbers: Vec<(Number, Pos)>,
    symbols: Vec<(Symbol, Pos)>,
}

impl FromStr for Line {
    type Err = ParseLineError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let length = s.len();
        let re_numbers = Regex::new(r"\d+").unwrap();
        let numbers = re_numbers
            .captures_iter(s)
            .map(|captures| {
                // todo unwrap
                let amatch = captures
                    .get(0)
                    .ok_or(ParseLineError::NumberParseError)
                    .unwrap();
                // todo unwrap
                let value = amatch
                    .as_str()
                    .parse::<u32>()
                    .map_err(|_| ParseLineError::NumberParseError)
                    .unwrap();
                let length = amatch.as_str().len();
                let pos = amatch.start();
                (Number { value, length }, Pos { pos })
            })
            .collect::<Vec<(Number, Pos)>>();

        let re_symbols = Regex::new(r"[^0-9\.]").unwrap();
        let symbols = re_symbols
            .captures_iter(s)
            .map(|captures| {
                // todo unwrap
                let amatch = captures
                    .get(0)
                    .ok_or(ParseLineError::SymbolParseError)
                    .unwrap();
                // todo unwrap
                let char = amatch
                    .as_str()
                    .chars()
                    .next()
                    .ok_or(ParseLineError::SymbolParseError)
                    .unwrap();
                let pos = amatch.start();
                (Symbol { char }, Pos { pos })
            })
            .collect::<Vec<(Symbol, Pos)>>();

        Ok(Line {
            length,
            numbers,
            symbols,
        })
    }
}

#[derive(Debug, PartialEq)]
enum ParseLineError {
    NumberParseError,
    SymbolParseError,
}

#[derive(Debug, PartialEq)]
struct Number {
    value: u32,
    length: usize,
}

#[derive(Debug, PartialEq)]
struct Symbol {
    char: char,
}

#[derive(Debug, PartialEq)]
struct Pos {
    pos: usize,
}

fn main() {
    let input = r"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    input.lines().for_each(|line| {
        println!("{:?}", line);
        println!("{:?}", line.parse::<Line>().unwrap());
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        let line_str = r"...123...$...";
        let line = line_str.parse::<Line>();
        assert_eq!(
            line,
            Ok(Line {
                length: 13,
                numbers: vec![(
                    Number {
                        value: 123,
                        length: 3
                    },
                    Pos { pos: 3 }
                ),],
                symbols: vec![(Symbol { char: '$' }, Pos { pos: 9 }),],
            })
        );
    }
}
