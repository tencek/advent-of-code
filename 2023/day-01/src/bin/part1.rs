use std::fmt;

fn main() {
    let input = include_str!("input1.txt");
    let calibration = parse_calibration(input);
    println!("Calibration: {}", calibration);
}

// new type representing the calibration
#[derive(Debug, PartialEq)]
struct Calibration(u32);

impl fmt::Display for Calibration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

fn read_digits(input: &str) -> Vec<u32> {
    input.chars().filter_map(|c| c.to_digit(10)).collect()
}

fn first_and_last(digits: &Vec<u32>) -> u32 {
    match digits.len() {
        0 => 0,
        _ => digits[0] * 10 + digits[digits.len() - 1],
    }
}

fn parse_calibration(input: &str) -> Calibration {
    // split the input into lines
    let result = input
        .lines()
        .into_iter()
        .map(|str| read_digits(str))
        .fold(0u32, |sum_so_far, digits| {
            sum_so_far + first_and_last(&digits)
        });
    Calibration(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_calibration() {
        let input = r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"#;
        let expected_output = Calibration(142);
        let result = parse_calibration(input);
        assert_eq!(result, expected_output);
    }

    #[test]
    fn test_read_digits() {
        let input = "1abc2";
        let expected_output = vec![1, 2];
        let result = read_digits(input);
        assert_eq!(result, expected_output);
    }
}
