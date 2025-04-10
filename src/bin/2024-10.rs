use std::collections::HashSet;

use advent_of_code::get_challenge_input_as_str;

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
struct Coordinate {
    pub x: usize,
    pub y: usize,
}

impl Coordinate {
    pub fn surrounding_coordinates(&self, map: &TopographicMap) -> Vec<Coordinate> {
        let surrounding_positions: Vec<Coordinate> = vec![
            Coordinate {
                x: self.x,
                y: self.y - 1,
            },
            Coordinate {
                x: self.x - 1,
                y: self.y,
            },
            Coordinate {
                x: self.x + 1,
                y: self.y,
            },
            Coordinate {
                x: self.x,
                y: self.y + 1,
            },
        ]
        .into_iter()
        .filter(|coordinate| {
            (0..map.width).contains(&(coordinate.x as i32))
                && (0..map.height).contains(&(coordinate.y as i32))
        })
        .collect();

        surrounding_positions
    }
}

struct TopographicMap {
    pub width: i32,
    pub height: i32,
    pub trailheads: HashSet<Coordinate>,
    pub data: MapData,
}

struct MapData(Vec<Vec<i32>>);

impl MapData {
    pub fn get(&self, coordinate: &Coordinate) -> i32 {
        self.0[coordinate.y][coordinate.x]
    }
}

fn parse_data(data: &str) -> Result<TopographicMap, String> {
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

    let height = data.lines().count() as i32;

    let trailheads: HashSet<Coordinate> = data
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, character)| *character == '0')
                .map(move |(column, _)| Coordinate { x: column, y: row })
        })
        .collect();

    let map: Vec<Vec<i32>> = data
        .lines()
        .enumerate()
        .map(|(_, line)| {
            line.chars()
                .enumerate()
                .map(|(_, c)| c.to_string().parse::<i32>().unwrap_or(99))
                .collect::<Vec<i32>>()
        })
        .collect();

    Ok(TopographicMap {
        width,
        height,
        trailheads,
        data: MapData(map),
    })
}

fn follow_trails(
    coordinate: Coordinate,
    map: &TopographicMap,
    next_height: i32,
    end_coordinates: &mut HashSet<Coordinate>,
) {
    let surrounding_coordinates = coordinate.surrounding_coordinates(map);

    match next_height {
        9 => {
            surrounding_coordinates
                .iter()
                .filter(|&coordinate| map.data.get(coordinate) == 9)
                .for_each(|&coordinate| {
                    end_coordinates.insert(coordinate);
                });
        }
        height if height > 0 && height < 9 => {
            surrounding_coordinates
                .iter()
                .filter(|coordinate| map.data.get(coordinate) == height)
                .for_each(|&coordinate| {
                    follow_trails(coordinate, map, height + 1, end_coordinates)
                });
        }
        _ => unreachable!(),
    }
}

fn follow_all_trails(
    coordinate: Coordinate,
    map: &TopographicMap,
    next_height: i32,
    trailhead_score: &mut i64,
) {
    let surrounding_coordinates = coordinate.surrounding_coordinates(map);

    match next_height {
        9 => {
            *trailhead_score += surrounding_coordinates
                .iter()
                .filter(|coordinate| map.data.get(coordinate) == 9)
                .count() as i64;
        }
        height if height > 0 && height < 9 => {
            surrounding_coordinates
                .iter()
                .filter(|coordinate| map.data.get(coordinate) == height)
                .for_each(|&coordinate| {
                    follow_all_trails(coordinate, map, height + 1, trailhead_score);
                });
        }
        _ => unreachable!(),
    }
}

#[allow(unused)]
fn part_one(data: &str) -> i64 {
    let mut result: i64 = 0;

    if let Ok(map) = parse_data(data) {
        for &trailhead in map.trailheads.iter() {
            let mut trail_ends: HashSet<Coordinate> = HashSet::new();

            follow_trails(trailhead, &map, 1, &mut trail_ends);

            result += trail_ends.len() as i64;
        }
    }

    result
}

#[allow(unused)]
fn part_two(data: &str) -> i64 {
    let mut result: i64 = 0;

    if let Ok(map) = parse_data(data) {
        for &trailhead in map.trailheads.iter() {
            let mut trailhead_score: i64 = 0;

            follow_all_trails(trailhead, &map, 1, &mut trailhead_score);
            result += trailhead_score;
        }
    }

    result
}

pub fn main() {
    if let Ok(data) = get_challenge_input_as_str(2024, 10) {
        let result = part_one(&data);
        println!("part one: {}", result);
        let result = part_two(&data);
        println!("part two: {}", result);
    }
}

mod tests {
    #![allow(unused)]
    use super::*;

    const EXAMPLE_INPUT: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    const ONE_TRAILHEAD_TWO_REACHABLE: &str = "...0...
...1...
...2...
6543456
7.....7
8.....8
9.....9";

    const ONE_TRAILHEAD_FOUR_REACHABLE: &str = "..90..9
...1.98
...2..7
6543456
765.987
876....
987....";

    #[test]
    fn test_parse() {
        let parsed = parse_data(EXAMPLE_INPUT);

        assert!(parsed.is_ok());
        assert_eq!(parsed.as_ref().unwrap().height, 8);
        assert_eq!(parsed.as_ref().unwrap().width, 8);
        assert_eq!(parsed.as_ref().unwrap().trailheads.len(), 9);
    }

    #[test]
    fn test_one_trailhead_two_reachable() {
        let result = part_one(ONE_TRAILHEAD_TWO_REACHABLE);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_one_trailhead_four_reachable() {
        let result = part_one(ONE_TRAILHEAD_FOUR_REACHABLE);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(EXAMPLE_INPUT);
        assert_eq!(result, 36);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(EXAMPLE_INPUT);
        assert_eq!(result, 81);
    }
}
