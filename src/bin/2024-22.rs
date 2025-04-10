use std::ops::BitXor;

use advent_of_code::get_challenge_input_as_str;

fn mix(value: u32, secret_number: u32) -> u32 {
    secret_number.bitxor(value)
}

fn prune(secret_number: u32) -> u32 {
    secret_number % 16777216
}

fn next_secret(secret_number: u32) -> u32 {
    let mut result = prune(mix(secret_number, secret_number * 64));
    result = prune(mix(result / 32, result));
    result = prune(mix(result * 2048, result));

    result
}

#[allow(unused)]
fn part_one(data: &str) -> u64 {
    let mut result: u64 = 0;

    for line in data.lines() {
        let mut secret: u32 = line.parse().unwrap();

        for i in 0..2000 {
            secret = next_secret(secret);
        }

        result += secret as u64;
    }

    result
}

#[allow(unused)]
fn part_two(data: &str) -> u64 {
    let mut result: u64 = 0;

    for line in data.lines() {
        let mut secret: u32 = line.parse().unwrap();
        let mut last_price: i32 = 0;

        for i in 0..2000 {
            secret = next_secret(secret);
            println!("{secret}: {} ({})", secret % 10,(secret % 10) as i32 - last_price);
            last_price = (secret % 10) as i32;
        }

        result += secret as u64;
    }

    result
}

pub fn main() {
    if let Ok(data) = get_challenge_input_as_str(2024, 22) {
        let result = part_one(&data);
        println!("part one: {}", result);
        let result = part_two(&data);
        println!("part two: {}", result);
    }
}

mod tests {
    #![allow(unused)]
    use super::*;

    const EXAMPLE_INPUT: &str = "1
10
100
2024";

    #[test]
    fn test_secret_number() {
        let mut secret = 123;
        let targets = [
            15887950, 16495136, 527345, 704524, 1553684, 12683156, 11100544, 12249484, 7753432,
            5908254,
        ];

        for i in targets {
            secret = next_secret(secret);
            assert_eq!(secret, i);
        }
    }

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(EXAMPLE_INPUT), 37327623);
    }

    #[test]
    fn test_part_two() {
        part_two("123");
    }
}
