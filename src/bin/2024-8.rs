use std::collections::{HashMap, HashSet};

use advent_of_code::{get_challenge_input_as_str, run_on_challenge_input_lines};
use itertools::Itertools;

struct Map {
    width: i32,
    height: i32,
    antenna_locations: HashMap<char, Vec<(f32, f32)>>,
}

fn parse_input(data: &str) -> Result<Map, String> {
    let width = data
        .lines()
        .next()
        .ok_or_else(|| "input data did not contain any lines".to_string())?
        .len() as i32;

    if !data.lines().all(|line| line.len() as i32 == width) {
        return Err(format!(
            "expected all lines to have the same length ({width})"
        ));
    }

    let antenna_locations: HashMap<char, Vec<(f32, f32)>> = data
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, character)| *character != '.')
                .map(move |(column, character)| (character, (column as f32, row as f32)))
        })
        .fold(HashMap::new(), |mut map, (character, coordinate)| {
            map.entry(character)
                .or_insert_with(Vec::new)
                .push(coordinate);

            map
        });

    let height = data.lines().count() as i32;

    Ok(Map {
        width,
        height,
        antenna_locations,
    })
}

fn distance_between_points(point_a: &(f32, f32), point_b: &(f32, f32)) -> f32 {
    ((point_b.0 - point_a.0).powi(2) + (point_b.1 - point_b.1).powi(2)).sqrt()
}

#[allow(unused)]
fn part_one(data: &str) -> i32 {
    let mut total_antinodes: HashSet<(i32, i32)> = HashSet::new();

    if let Ok(map) = parse_input(data) {
        for (frequency, positions) in map.antenna_locations.iter() {
            for pair in positions.iter().combinations(2) {
                let antenna_a_position = pair[0];
                let antenna_b_position = pair[1];

                let distance = distance_between_points(antenna_a_position, antenna_b_position);
                let direction_x = (antenna_b_position.0 - antenna_a_position.0) / distance;
                let direction_y = (antenna_b_position.1 - antenna_a_position.1) / distance;

                let antinode_a = (
                    (antenna_a_position.0 - direction_x * distance) as i32,
                    (antenna_a_position.1 - direction_y * distance) as i32,
                );

                let antinode_b = (
                    (antenna_b_position.0 + direction_x * distance) as i32,
                    (antenna_b_position.1 + direction_y * distance) as i32,
                );

                total_antinodes.insert(antinode_a);
                total_antinodes.insert(antinode_b);
            }
        }

        println!(
            "{:?}",
            total_antinodes
                .iter()
                .filter(|(x, y)| (0..map.width).contains(x) && (0..map.height).contains(y))
                .collect::<Vec<_>>()
        );

        return total_antinodes
            .iter()
            .filter(|(x, y)| (0..map.width).contains(x) && (0..map.height).contains(y))
            .count() as i32;
    }

    return 0;
}

#[allow(unused)]
fn part_two(data: &str) -> i32 {
    let mut total_antinodes: HashSet<(i32, i32)> = HashSet::new();

    if let Ok(map) = parse_input(data) {
        for (frequency, positions) in map.antenna_locations.iter() {
            for pair in positions.iter().combinations(2) {
                let antenna_a_position = pair[0];
                let antenna_b_position = pair[1];

                let row_diff = antenna_a_position.0 - antenna_b_position.0;
                let col_diff = antenna_a_position.1 - antenna_b_position.1;

                // total_antinodes.insert((antenna_a_position.0 as i32, antenna_a_position.1 as i32));
                // total_antinodes.insert((antenna_b_position.0 as i32, antenna_b_position.1 as i32));

                // let distance = distance_between_points(antenna_a_position, antenna_b_position);
                // let direction_x = (antenna_b_position.0 - antenna_a_position.0) / distance;
                // let direction_y = (antenna_b_position.1 - antenna_a_position.1) / distance;

                for d in 1..map.height {
                    let antinode_a = (
                        (antenna_a_position.0 - row_diff * (d as f32)) as i32,
                        (antenna_a_position.1 - col_diff * (d as f32)) as i32,
                    );

                    let antinode_b = (
                        (antenna_b_position.0 + row_diff * (d as f32)) as i32,
                        (antenna_b_position.1 + col_diff * (d as f32)) as i32,
                    );

                    total_antinodes.insert(antinode_a);
                    total_antinodes.insert(antinode_b);
                }
            }
        }

        println!(
            "{:?}",
            total_antinodes
                .iter()
                .filter(|(x, y)| (0..map.width).contains(x) && (0..map.height).contains(y))
                .collect::<Vec<_>>()
        );

        return total_antinodes
            .iter()
            .filter(|(x, y)| (0..map.width).contains(x) && (0..map.height).contains(y))
            .count() as i32;
    }

    return 0;
}

pub fn main() {
    if let Ok(data) = get_challenge_input_as_str(2024, 8) {
        let result = part_one(&data);
        println!("part one: {}", result);

        let result = part_two(&data);
        println!("part two: {}", result);
    }
}

mod tests {
    #![allow(unused)]
    use super::*;

    const EXAMPLE_INPUT: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    #[test]
    fn test_parse_input() {
        let map = parse_input(EXAMPLE_INPUT);

        assert!(map.is_ok());

        let map = map.unwrap();
        assert!(map.width == 12);
        assert!(map.height == 12);
        assert!(map.antenna_locations.keys().len() == 2);
    }

    #[test]
    fn test_part_one() {
        let antinodes = part_one(EXAMPLE_INPUT);

        assert_eq!(antinodes, 14);
    }

    #[test]
    fn test_part_two() {
        let antinodes = part_two(EXAMPLE_INPUT);

        assert_eq!(antinodes, 34);
    }
}
