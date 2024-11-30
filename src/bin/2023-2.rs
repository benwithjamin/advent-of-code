use std::fmt::Debug;

use advent_of_code::run_on_challenge_input;
use regex::Regex;

struct Game {
    pub number: u64,
    pub red: Vec<u64>,
    pub green: Vec<u64>,
    pub blue: Vec<u64>,
}

impl Debug for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Game")
            .field("number", &self.number)
            .field("red", &self.red)
            .field("green", &self.green)
            .field("blue", &self.blue)
            .finish()
    }
}

fn parse_game(value: String) -> Game {
    let regex_game_number = Regex::new(r"Game (\d+)").unwrap();
    let regex_red = Regex::new(r"(\d+) red").unwrap();
    let regex_green = Regex::new(r"(\d+) green").unwrap();
    let regex_blue = Regex::new(r"(\d+) blue").unwrap();

    let mut red_values: Vec<u64> = regex_red
        .captures_iter(&value)
        .filter_map(|capture| capture.get(1))
        .filter_map(|m| m.as_str().parse::<u64>().ok())
        .collect();

    red_values.sort_by(|a, b| b.cmp(a));

    let mut green_values: Vec<u64> = regex_green
        .captures_iter(&value)
        .filter_map(|capture| capture.get(1))
        .filter_map(|m| m.as_str().parse::<u64>().ok())
        .collect();

    green_values.sort_by(|a, b| b.cmp(a));

    let mut blue_values: Vec<u64> = regex_blue
        .captures_iter(&value)
        .filter_map(|capture| capture.get(1))
        .filter_map(|m| m.as_str().parse::<u64>().ok())
        .collect();

    blue_values.sort_by(|a, b| b.cmp(a));

    Game {
        number: regex_game_number
            .captures(&value)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .parse::<u64>()
            .unwrap(),
        red: red_values,
        green: green_values,
        blue: blue_values,
    }
}

fn part_one(value: impl Into<String>) -> bool {
    let game: Game = parse_game(value.into());
    !(game.red[0] > 12 || game.green[0] > 13 || game.blue[0] > 14)
}

fn part_two(value: impl Into<String>) -> u64 {
    let game: Game = parse_game(value.into());
    game.red[0] * game.green[0] * game.blue[0]
}

pub fn main() {
    let mut valid_games: Vec<u64> = vec![];
    let mut game_index = 1;

    run_on_challenge_input(2023, 2, |line| {
        if part_one(line) {
            valid_games.push(game_index);
        }

        game_index += 1;
    });

    let answer: u64 = valid_games.iter().sum();
    println!("Part one: {}", answer);

    let mut game_powers = 0;

    run_on_challenge_input(2023, 2, |line| {
        game_powers += part_two(line);
    });

    println!("Part two: {}", game_powers);
}
