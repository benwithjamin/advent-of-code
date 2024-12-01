use advent_of_code::run_on_challenge_input;
use regex::Regex;

fn part_one() {
    let mut left: Vec<u64> = vec![];
    let mut right: Vec<u64> = vec![];

    let regex = Regex::new(r"(\d{5})\s+(\d{5})").unwrap();

    run_on_challenge_input(2024, 1, |line| {
        let captures = regex.captures(line).unwrap();
        left.push(captures.get(1).unwrap().as_str().parse::<u64>().unwrap());
        right.push(captures.get(2).unwrap().as_str().parse::<u64>().unwrap());
    });

    left.sort();
    right.sort();

    let mut total: u64 = 0;

    for i in 0..left.len() {
        total += left[i].abs_diff(right[i]);
    }

    println!("part one: {}", total);
}

fn part_two() {
    let mut left: Vec<u64> = vec![];
    let mut right: Vec<u64> = vec![];

    let regex = Regex::new(r"(\d{5})\s+(\d{5})").unwrap();

    run_on_challenge_input(2024, 1, |line| {
        let captures = regex.captures(line).unwrap();
        left.push(captures.get(1).unwrap().as_str().parse::<u64>().unwrap());
        right.push(captures.get(2).unwrap().as_str().parse::<u64>().unwrap());
    });

    left.sort();
    right.sort();

    let mut total: u64 = 0;

    for i in 0..left.len() {
        total += left[i] * right.iter().filter(|value| *value == &left[i]).count() as u64;
    }

    println!("part two: {}", total);
}

pub fn main() {
    part_one();
    part_two();
}
