use regex::Regex;
use std::ops::ControlFlow;
use std::str::FromStr;

struct GameConfig {
    red_cubes: u32,
    green_cubes: u32,
    blue_cubes: u32,
}

impl GameConfig {
    fn is_valid_game_run(&self, game_run: &GameRun) -> bool {
        game_run.draws.iter().all(|draw| {
            let red_cubes = draw.red_cubes.unwrap_or(0);
            let green_cubes = draw.green_cubes.unwrap_or(0);
            let blue_cubes = draw.blue_cubes.unwrap_or(0);
            red_cubes <= self.red_cubes
                && green_cubes <= self.green_cubes
                && blue_cubes <= self.blue_cubes
        })
    }
}

#[derive(Debug, PartialEq)]
enum CubeColor {
    Red,
    Green,
    Blue,
}

#[derive(Debug, PartialEq)]
struct Draw {
    red_cubes: Option<u32>,
    green_cubes: Option<u32>,
    blue_cubes: Option<u32>,
}

#[derive(Debug)]
struct GameRun {
    id: u32,
    draws: Vec<Draw>,
}

#[derive(Debug, PartialEq)]
enum GameParseError {
    InvalidFormat,
    InvalidGameId(String),
    InvalidDraw(String),
    InvalidColor(String),
    ColorListedTwice(CubeColor),
    InvalidCount(String),
}

impl FromStr for CubeColor {
    type Err = GameParseError;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "red" => Ok(CubeColor::Red),
            "green" => Ok(CubeColor::Green),
            "blue" => Ok(CubeColor::Blue),
            color => Err(GameParseError::InvalidColor(color.to_string())),
        }
    }
}

impl FromStr for Draw {
    type Err = GameParseError;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let result = input
            .split(",")
            .into_iter()
            .map(|s| {
                Regex::new(r"(\d+) (\w+)")
                    .unwrap()
                    .captures(s.trim())
                    .ok_or(GameParseError::InvalidFormat)
                    .and_then(|captures| {
                        let color = captures
                            .get(2)
                            .ok_or(GameParseError::InvalidFormat)
                            .and_then(|a_match| a_match.as_str().parse::<CubeColor>());
                        let count = captures
                            .get(1)
                            .ok_or(GameParseError::InvalidFormat)
                            .and_then(|a_match| {
                                a_match.as_str().parse::<u32>().map_err(|parse_int_error| {
                                    GameParseError::InvalidCount(parse_int_error.to_string())
                                })
                            });
                        match (color, count) {
                            (Ok(color), Ok(count)) => Ok((color, count)),
                            (Err(game_parse_error), _) => Err(game_parse_error),
                            (_, Err(game_parse_error)) => Err(game_parse_error),
                        }
                    })
            })
            .collect::<Result<Vec<(CubeColor, u32)>, GameParseError>>()
            .map(|vec| {
                vec.into_iter().try_fold(
                    Draw {
                        red_cubes: None,
                        green_cubes: None,
                        blue_cubes: None,
                    },
                    |draw, (color, count)| match color {
                        CubeColor::Red => {
                            if draw.red_cubes.is_none() {
                                ControlFlow::Continue(Draw {
                                    red_cubes: Some(count),
                                    ..draw
                                })
                            } else {
                                ControlFlow::Break(GameParseError::ColorListedTwice(color))
                            }
                        }
                        CubeColor::Green => {
                            if draw.green_cubes.is_none() {
                                ControlFlow::Continue(Draw {
                                    green_cubes: Some(count),
                                    ..draw
                                })
                            } else {
                                ControlFlow::Break(GameParseError::ColorListedTwice(color))
                            }
                        }
                        CubeColor::Blue => {
                            if draw.blue_cubes.is_none() {
                                ControlFlow::Continue(Draw {
                                    blue_cubes: Some(count),
                                    ..draw
                                })
                            } else {
                                ControlFlow::Break(GameParseError::ColorListedTwice(color))
                            }
                        }
                    },
                )
            });

        match result {
            Ok(ControlFlow::Continue(draw)) => Ok(draw),
            Ok(ControlFlow::Break(game_parse_error)) => Err(game_parse_error),
            Err(game_parse_error) => Err(game_parse_error),
        }
    }
}

impl FromStr for GameRun {
    type Err = GameParseError;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        regex::Regex::new(r"Game (\w+):(.*)")
            .unwrap()
            .captures(input)
            .ok_or(GameParseError::InvalidFormat)
            .and_then(|captures| {
                let game_id = captures
                    .get(1)
                    .ok_or(GameParseError::InvalidGameId(String::from(
                        "Game id not found",
                    )))
                    .and_then(|a_match| {
                        a_match.as_str().parse::<u32>().map_err(|_| {
                            GameParseError::InvalidGameId(a_match.as_str().to_string())
                        })
                    });

                game_id.and_then(|game_id| {
                    let draws = captures
                        .get(2)
                        .map(|m| m.as_str())
                        .ok_or(GameParseError::InvalidDraw(String::from("Draws not found")))
                        .and_then(GameRun::parse_draws);

                    draws.map(|draws| GameRun {
                        id: game_id,
                        draws: draws,
                    })
                })
            })
    }
}

impl GameRun {
    fn parse_draws(input: &str) -> Result<Vec<Draw>, GameParseError> {
        input.split(";").into_iter().map(Draw::from_str).collect()
    }
}

fn part1(inputdata: &str) -> u32 {
    let game_config = GameConfig {
        red_cubes: 12,
        green_cubes: 13,
        blue_cubes: 14,
    };
    inputdata
        .lines()
        .into_iter()
        .map(|s| s.parse::<GameRun>().unwrap())
        .filter(|game_run| game_config.is_valid_game_run(game_run))
        .map(|game_run| game_run.id)
        .sum::<u32>()
}

fn part2(inputdata: &str) -> u32 {
    inputdata
        .lines()
        .into_iter()
        .map(|s| s.parse::<GameRun>().unwrap())
        .fold(0u32, |acc, game_run| {
            let max_red = game_run
                .draws
                .iter()
                .map(|draw| draw.red_cubes.unwrap_or(0))
                .max()
                .unwrap_or(0);
            let max_green = game_run
                .draws
                .iter()
                .map(|draw| draw.green_cubes.unwrap_or(0))
                .max()
                .unwrap_or(0);
            let max_blue = game_run
                .draws
                .iter()
                .map(|draw| draw.blue_cubes.unwrap_or(0))
                .max()
                .unwrap_or(0);
            acc + max_red * max_green * max_blue
        })
}

fn main() {
    let result_part1 = part1(include_str!("input.txt"));
    println!("part1: {}", result_part1);

    let result_part2 = part2(include_str!("input.txt"));
    println!("part2: {}", result_part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let test_data = r"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let expected_part1_result = 1 + 2 + 5;
        let part1_result = part1(test_data);
        assert_eq!(part1_result, expected_part1_result);
    }

    #[test]
    fn test_part2() {
        let test_data = r"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let expected_part2_result = 2286;
        let part2_result = part2(test_data);
        assert_eq!(part2_result, expected_part2_result);
    }

    #[test]
    fn test_parse_game_run() {
        let str = String::from("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");
        let game_run = str.parse::<GameRun>().unwrap();
        assert_eq!(game_run.id, 1);
        assert_eq!(game_run.draws.len(), 3);
        assert_eq!(game_run.draws[0].red_cubes, Some(4));
        assert_eq!(game_run.draws[0].green_cubes, None);
        assert_eq!(game_run.draws[0].blue_cubes, Some(3));
        assert_eq!(game_run.draws[1].red_cubes, Some(1));
        assert_eq!(game_run.draws[1].green_cubes, Some(2));
        assert_eq!(game_run.draws[1].blue_cubes, Some(6));
        assert_eq!(game_run.draws[2].red_cubes, None);
        assert_eq!(game_run.draws[2].green_cubes, Some(2));
        assert_eq!(game_run.draws[2].blue_cubes, None);
    }

    #[test]
    fn test_parse_game_run_invalid_game_id() {
        let str = String::from("Game 1a: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");
        let game_run = str.parse::<GameRun>();
        assert!(game_run.is_err());
        assert_eq!(
            game_run.unwrap_err(),
            GameParseError::InvalidGameId(String::from("1a",))
        );
    }

    #[test]
    fn test_parse_game_run_invalid_draw() {
        let str = String::from("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green, 1 yellow");
        let game_run = str.parse::<GameRun>();
        assert!(game_run.is_err());
        assert_eq!(
            game_run.unwrap_err(),
            GameParseError::InvalidColor(String::from("yellow"))
        );
    }
}
