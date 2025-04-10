use std::collections::HashMap;

use advent_of_code::get_challenge_input_as_str;

fn parse_data(data: &str) -> Vec<i64> {
    data.split(" ")
        .filter(|stone| !stone.is_empty())
        .map(|stone| stone.parse::<i64>())
        .map(Result::unwrap)
        .collect::<Vec<i64>>()
}

fn count_digits(number: i64) -> i64 {
    format!("{}", number).len() as i64
}

fn part_one_step(stones: &mut Vec<i64>) {
    let mut index = 0;

    while index <= stones.len() - 1 {
        let stone = stones[index];

        match stone {
            0 => {
                stones[index] = 1;
            }
            i if count_digits(i) % 2 == 0 => {
                let stone_str = format!("{}", stone).to_string();
                let (left, right) = stone_str.split_at(stone_str.len() / 2);

                let left_stone: i64 = left.parse::<i64>().expect("Error parsing left");
                let right_stone: i64 = right.parse::<i64>().expect("Error parsing right");

                stones.splice(index..=index, vec![left_stone, right_stone]);

                index += 1;
            }
            _ => {
                stones[index] *= 2024;
            }
        }

        index += 1;
    }
}

fn part_two_step(stones: &mut HashMap<i64, usize>) {
    let mut updated_stones: HashMap<i64, usize> = HashMap::new();

    for (&stone, count) in stones.iter() {
        let stone_digits = count_digits(stone);
        let half_digits = stone_digits / 2;
        let divisor = 10_i64.pow(half_digits as u32);

        match stone {
            0 => {
                *updated_stones.entry(1).or_default() += count
            },
            _ if stone_digits % 2 == 0 => {
                *updated_stones.entry(stone / divisor).or_default() += count;
                *updated_stones.entry(stone % divisor).or_default() += count;
            },
            _ => {
                *updated_stones.entry(stone * 2024).or_default() += count;
            }
        }
    }

    stones.clear();
    stones.clone_from(&updated_stones);
}


#[allow(unused)]
fn part_one(data: &str, steps: i64) -> i64 {
    let mut stones = parse_data(data);

    for step in 0..steps {
        part_one_step(&mut stones);
    }

    stones.len() as i64
}

#[allow(unused)]
fn part_two(data: &str, steps: i64) -> i64 {
    let mut stone_data = parse_data(data);
    let mut stones: HashMap<i64, usize> = HashMap::new();
    let mut stone_count: i64 = 0;

    for &stone in stone_data.iter() {
        *stones.entry(stone).or_default() += 1;
    }

    for step in 0..steps {
        part_two_step(&mut stones);
    }

    for (_, count) in stones {
        stone_count += count as i64;
    }

    stone_count
}

pub fn main() {
    if let Ok(data) = get_challenge_input_as_str(2024, 11) {
        let result = part_one(&data, 25);
        println!("part one: {}", result);
        let result = part_two(&data, 75);
        println!("part two: {}", result);
    }
}

mod tests {
    #![allow(unused)]
    use super::*;

    const EXAMPLE_INPUT: &str = "125 17";

    #[test]
    fn test_parse() {
        let data = parse_data(EXAMPLE_INPUT);

        assert_eq!(data, vec![125, 17]);
    }

    #[test]
    fn test_part_one() {
        let mut stones = parse_data(EXAMPLE_INPUT);

        part_one_step(&mut stones);
        assert_eq!(stones, vec![253000, 1, 7]);
        part_one_step(&mut stones);
        assert_eq!(stones, vec![253, 0, 2024, 14168]);
        part_one_step(&mut stones);
        assert_eq!(stones, vec![512072, 1, 20, 24, 28676032]);
        part_one_step(&mut stones);
        assert_eq!(stones, vec![512, 72, 2024, 2, 0, 2, 4, 2867, 6032]);
        part_one_step(&mut stones);
        assert_eq!(
            stones,
            vec![1036288, 7, 2, 20, 24, 4048, 1, 4048, 8096, 28, 67, 60, 32]
        );
        part_one_step(&mut stones);
        assert_eq!(
            stones,
            vec![
                2097446912, 14168, 4048, 2, 0, 2, 4, 40, 48, 2024, 40, 48, 80, 96, 2, 8, 6, 7, 6,
                0, 3, 2
            ]
        );
    }

    #[test]
    fn test_part_two() {}
}
