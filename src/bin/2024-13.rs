#![allow(unused)]
use std::{collections::HashSet, fmt::Debug};

use advent_of_code::{get_challenge_input_as_str, maps::Coordinate};
use itertools::Itertools;
use regex::Regex;

#[derive(Debug, Copy, Clone)]
struct ButtonConfig {
    x_step: i64,
    y_step: i64,
}

#[derive(Debug, Copy, Clone)]
pub struct ClawMachineConfig {
    button_a: ButtonConfig,
    button_b: ButtonConfig,
    prize_location: Coordinate<i64>,
}

fn parse_data(data: &str) -> Result<Vec<ClawMachineConfig>, String> {
    let parse_regex =
        Regex::new(r"(Button [AB]|Prize): (X(\+[0-9]+), Y(\+[0-9]+)|X=([0-9]+), Y=([0-9]+))")
            .unwrap();
    let mut machine_configs: Vec<ClawMachineConfig> = Vec::new();

    for block in data.lines().chunks(4).into_iter() {
        let mut button_a: ButtonConfig = ButtonConfig {
            x_step: 0,
            y_step: 0,
        };
        let mut button_b: ButtonConfig = ButtonConfig {
            x_step: 0,
            y_step: 0,
        };
        let mut prize_location = Coordinate { x: 0i64, y: 0i64 };

        for line in block {
            if line.is_empty() {
                continue;
            }

            if let Some(captures) = parse_regex.captures(line) {
                if let Some(line_type) = captures.get(1) {
                    match line_type.as_str() {
                        "Button A" => {
                            button_a.x_step =
                                captures.get(3).unwrap().as_str().parse::<i64>().unwrap();
                            button_a.y_step =
                                captures.get(4).unwrap().as_str().parse::<i64>().unwrap();
                        }
                        "Button B" => {
                            button_b.x_step =
                                captures.get(3).unwrap().as_str().parse::<i64>().unwrap();
                            button_b.y_step =
                                captures.get(4).unwrap().as_str().parse::<i64>().unwrap();
                        }
                        "Prize" => {
                            prize_location.x =
                                captures.get(5).unwrap().as_str().parse::<i64>().unwrap();
                            prize_location.y =
                                captures.get(6).unwrap().as_str().parse::<i64>().unwrap();
                        }
                        _ => return Err("Line is malformed (line type incorrect)".to_string()),
                    }
                } else {
                    return Err("Line is malformed (no line type)".to_string());
                }
            }
        }

        let machine_config = ClawMachineConfig {
            button_a,
            button_b,
            prize_location,
        };

        machine_configs.push(machine_config);
    }

    Ok(machine_configs)
}

fn solve_machine(config: ClawMachineConfig, part_two: bool) -> Option<i64> {
    let prize_location = match part_two {
        true => Coordinate {
            x: config.prize_location.x + 10000000000000,
            y: config.prize_location.y + 10000000000000,
        },
        false => Coordinate {
            x: config.prize_location.x,
            y: config.prize_location.y,
        },
    };

    let a = (prize_location.x * config.button_b.y_step - prize_location.y * config.button_b.x_step)
        / (config.button_a.x_step * config.button_b.y_step
            - config.button_a.y_step * config.button_b.x_step);

    let b = (config.button_a.x_step * prize_location.y - config.button_a.y_step * prize_location.x)
        / (config.button_a.x_step * config.button_b.y_step
            - config.button_a.y_step * config.button_b.x_step);

    if (
        config.button_a.x_step * a + config.button_b.x_step * b,
        config.button_a.y_step * a + config.button_b.y_step * b,
    ) == (prize_location.x, prize_location.y)
    {
        return Some(a * 3 + b);
    }

    None
}

#[allow(unused)]
fn part_one(machine_configs: &Vec<ClawMachineConfig>) -> i64 {
    machine_configs
        .iter()
        .filter_map(|&config| solve_machine(config, false))
        .sum()
}

#[allow(unused)]
fn part_two(machine_configs: &Vec<ClawMachineConfig>) -> i64 {
    machine_configs
        .iter()
        .filter_map(|&config| solve_machine(config, true))
        .sum()
}

pub fn main() {
    if let Ok(data) = get_challenge_input_as_str(2024, 13) {
        if let Ok(machine_configs) = parse_data(&data) {
            let result = part_one(machine_configs.as_ref());
            println!("part one: {}", result);
            let result = part_two(machine_configs.as_ref());
            println!("part two: {}", result);
        }
    }
}

mod tests {
    #![allow(unused)]
    use super::*;

    const EXAMPLE_INPUT: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

    #[test]
    fn test_parse() {
        let configs = parse_data(EXAMPLE_INPUT);

        assert!(configs.is_ok());
        assert_eq!(configs.as_ref().unwrap().len(), 4);
    }

    #[test]
    fn test_part_one() {
        let configs = parse_data(EXAMPLE_INPUT).unwrap();

        assert!(solve_machine(configs[0], false).is_some_and(|tokens| tokens == 280));
        assert!(solve_machine(configs[1], false).is_none());
        assert!(solve_machine(configs[2], false).is_some_and(|tokens| tokens == 200));
        assert!(solve_machine(configs[3], false).is_none());
    }

    #[test]
    fn test_part_two() {
        let configs = parse_data(EXAMPLE_INPUT).unwrap();

        assert!(solve_machine(configs[0], true).is_some_and(|tokens| tokens == 280));
        assert!(solve_machine(configs[1], true).is_none());
        assert!(solve_machine(configs[2], true).is_some_and(|tokens| tokens == 200));
        assert!(solve_machine(configs[3], true).is_none());
    }
}
