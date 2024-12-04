use advent_of_code::{run_on_challenge_input_lines, utils::split_keep};
use regex::Regex;

fn sum_uncorrupted_instructions(line: &str) -> u64 {
    let re = Regex::new(r"mul\(([0-9]{1,3})\,([0-9]{1,3})\)").unwrap();

    let total: u64 = re
        .captures_iter(&line)
        .map(|c| {
            c.get(1).unwrap().as_str().parse::<u64>().unwrap()
                * c.get(2).unwrap().as_str().parse::<u64>().unwrap()
        })
        .sum();

    total
}

fn sum_uncorrupted_enabled_instructions(line: &str, enabled: &mut bool) -> u64 {
    let re = Regex::new(r"don\'t\(\)|do\(\)").unwrap();

    let mut total: u64 = 0;

    for m in split_keep(&re, line) {
        match m {
            "don't()" => *enabled = false,
            "do()" => *enabled = true,
            _ => {
                if *enabled {
                    total += sum_uncorrupted_instructions(m)
                }
            }
        }
    }

    total
}

fn part_one() {
    let mut total: u64 = 0;

    run_on_challenge_input_lines(2024, 3, |line| {
        total += sum_uncorrupted_instructions(line);
    });

    println!("part one: {}", total);
}

fn part_two() {
    let mut total: u64 = 0;
    let mut enabled = true;

    run_on_challenge_input_lines(2024, 3, |line| {
        total += sum_uncorrupted_enabled_instructions(line, &mut enabled);
    });

    println!("part two: {}", total);
}

pub fn main() {
    part_one();
    part_two();
}
