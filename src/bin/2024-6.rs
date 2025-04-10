use std::{collections::HashSet, hash::Hash};

use advent_of_code::{get_challenge_input_as_str, run_on_challenge_input_lines};

#[derive(Debug, PartialEq, Eq)]
struct Map {
    width: i32,
    height: i32,
    obstructions: HashSet<(i32, i32)>,
    guard: (i32, i32),
}

fn parse_map(map: &str) -> Result<Map, String> {
    let width = map
        .lines()
        .next()
        .ok_or_else(|| "input data did not contain any lines".to_string())?
        .len() as i32;

    if !map.lines().all(|line| line.len() as i32 == width) {
        return Err(format!(
            "expected all lines to have the same length ({width})"
        ));
    }

    let height = map.lines().count() as i32;

    let guard = map
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c == '^')
                .map(move |(column, _)| (column as i32, row as i32))
        })
        .next()
        .ok_or_else(|| "unable to find guard in input".to_string())?;

    let obstructions: HashSet<(i32, i32)> = map
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(move |(column, _)| (column as i32, row as i32))
        })
        .collect();

    Ok(Map {
        width,
        height,
        obstructions,
        guard,
    })
}

// fn predict_guard_movements(map: &Map) -> HashSet<(i32, i32)> {
//     let mut position = map.guard;
//     let mut direction = (0, -1);
//     let mut visited: HashSet<(i32, i32)> =
//         HashSet::with_capacity((map.width * map.height) as usize);

//     while position.0 >= 0 && position.1 >= 0 && position.0 <= map.width && position.1 <= map.height {
//         visited.insert(position);

//         if map.obstructions.contains(&(position.0 + direction.0, position.1 + direction.1))
//     }
// }

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum GuardOrientation {
    North,
    East,
    South,
    West,
}

#[derive(Debug)]
enum GuardMovementOutcome {
    Advance,
    Blocked,
    Exit,
}

fn find_guard(map: &Vec<Vec<char>>) -> Option<(i64, i64)> {
    let mut line_index = 0;

    for line in map {
        for (position, &character) in line.iter().enumerate() {
            if character == '^' {
                return Some((line_index, position as i64));
            }
        }

        line_index += 1;
    }

    None
}

fn predict_guard_move(
    guard_location: (i64, i64),
    map: &mut Vec<Vec<char>>,
    orientation: GuardOrientation,
) -> GuardMovementOutcome {
    let next_coordinates: (i64, i64) = match orientation {
        GuardOrientation::North => (guard_location.0 - 1, guard_location.1),
        GuardOrientation::East => (guard_location.0, guard_location.1 + 1),
        GuardOrientation::South => (guard_location.0 + 1, guard_location.1),
        GuardOrientation::West => (guard_location.0, guard_location.1 - 1),
    };

    if next_coordinates.0 < 0
        || next_coordinates.1 < 0
        || next_coordinates.0 > (map[0].len() as i64) - 1
        || next_coordinates.1 > (map[0].len() as i64) - 1
    {
        map[guard_location.0 as usize][guard_location.1 as usize] = match orientation {
            GuardOrientation::North | GuardOrientation::South => '|',
            GuardOrientation::East | GuardOrientation::West => '-',
        };
        return GuardMovementOutcome::Exit;
    }

    if map[next_coordinates.0 as usize][next_coordinates.1 as usize] == '#' {
        map[guard_location.0 as usize][guard_location.1 as usize] = '+';
        return GuardMovementOutcome::Blocked;
    }

    GuardMovementOutcome::Advance
}

fn next_guard_location(guard_location: (i64, i64), orientation: &GuardOrientation) -> (i64, i64) {
    match orientation {
        GuardOrientation::North => (guard_location.0 - 1, guard_location.1),
        GuardOrientation::East => (guard_location.0, guard_location.1 + 1),
        GuardOrientation::South => (guard_location.0 + 1, guard_location.1),
        GuardOrientation::West => (guard_location.0, guard_location.1 - 1),
    }
}

fn advance_guard(
    guard_location: (i64, i64),
    map: &mut Vec<Vec<char>>,
    orientation: GuardOrientation,
) -> (i64, i64) {
    let new_location: (i64, i64) = next_guard_location(guard_location, &orientation);

    match map[guard_location.0 as usize][guard_location.1 as usize] {
        '.' => {
            map[guard_location.0 as usize][guard_location.1 as usize] = match &orientation {
                GuardOrientation::North | GuardOrientation::South => '|',
                GuardOrientation::East | GuardOrientation::West => '-',
            };
        }
        '|' => {
            if orientation == GuardOrientation::East || orientation == GuardOrientation::West {
                map[guard_location.0 as usize][guard_location.1 as usize] = '+';
            }
        }
        '-' => {
            if orientation == GuardOrientation::South || orientation == GuardOrientation::North {
                map[guard_location.0 as usize][guard_location.1 as usize] = '+';
            }
        }
        _ => {}
    }

    new_location
}

fn count_visited(map: &Vec<Vec<char>>) -> i64 {
    let mut count: i64 = 0;

    for line in map {
        for &character in line.iter() {
            if character == '|' || character == '-' || character == '+' {
                count += 1;
            }
        }
    }

    count
}

#[allow(unused)]
fn part_one() {
    let mut map: Vec<Vec<char>> = vec![];
    let mut guard_location: Option<(i64, i64)> = None;
    let mut guard_orientation: GuardOrientation = GuardOrientation::North;

    run_on_challenge_input_lines(2024, 6, |line| {
        map.push(line.chars().map(|c| c).collect());
    });

    if let Some(mut location) = find_guard(&map) {
        map[location.0 as usize][location.1 as usize] = '|';
        loop {
            match predict_guard_move(location, &mut map, guard_orientation) {
                GuardMovementOutcome::Advance => {
                    location = advance_guard(location, &mut map, guard_orientation)
                }
                GuardMovementOutcome::Blocked => {
                    guard_orientation = match guard_orientation {
                        GuardOrientation::North => GuardOrientation::East,
                        GuardOrientation::East => GuardOrientation::South,
                        GuardOrientation::South => GuardOrientation::West,
                        GuardOrientation::West => GuardOrientation::North,
                    };
                    location = advance_guard(location, &mut map, guard_orientation);
                }
                GuardMovementOutcome::Exit => break,
                _ => {}
            }
        }
    }

    println!("part one: {}", count_visited(&map));

    for line in map {
        println!("{}", line.into_iter().collect::<String>());
    }
}

/**
 * Calculate all visited then try each of those
 */
#[allow(unused)]
fn part_two() {
    let mut map: Vec<Vec<char>> = vec![];
    let mut guard_orientation: GuardOrientation = GuardOrientation::North;
    let mut visited: Vec<(i64, i64)> = vec![];
    let mut visited_locations: HashSet<(i64, i64, GuardOrientation)> = HashSet::new();

    run_on_challenge_input_lines(2024, 6, |line| {
        map.push(line.chars().map(|c| c).collect());
    });

    if let Some(mut location) = find_guard(&map) {
        map[location.0 as usize][location.1 as usize] = '|';
        loop {
            match predict_guard_move(location, &mut map, guard_orientation) {
                GuardMovementOutcome::Advance => {
                    visited_locations.insert((location.0, location.1, guard_orientation));
                    location = advance_guard(location, &mut map, guard_orientation);
                }
                GuardMovementOutcome::Blocked => {
                    visited_locations.insert((location.0, location.1, guard_orientation));

                    guard_orientation = match guard_orientation {
                        GuardOrientation::North => GuardOrientation::East,
                        GuardOrientation::East => GuardOrientation::South,
                        GuardOrientation::South => GuardOrientation::West,
                        GuardOrientation::West => GuardOrientation::North,
                    };
                    location = advance_guard(location, &mut map, guard_orientation);
                }
                GuardMovementOutcome::Exit => break,
                _ => {}
            }
        }
    }

    for x in 0..map.len() {
        for y in 0..map[0].len() {
            if map[x][y] == '|' || map[x][y] == '-' || map[x][y] == '+' {
                visited.push((x as i64, y as i64));
            }
        }
    }

    let mut loop_count = 0;

    for &visited_point in visited.iter() {
        map.clear();
        run_on_challenge_input_lines(2024, 6, |line| {
            map.push(line.chars().map(|c| c).collect());
        });

        guard_orientation = GuardOrientation::North;

        // Skip initial position
        if let Some(mut location) = find_guard(&map) {
            if location == visited_point {
                println!("skip");
                continue;
            }
        }

        let mut step_count = 0;

        map[visited_point.0 as usize][visited_point.1 as usize] = '#';

        if let Some(mut location) = find_guard(&map) {
            map[location.0 as usize][location.1 as usize] = '|';
            loop {
                match predict_guard_move(location, &mut map, guard_orientation) {
                    GuardMovementOutcome::Advance => {
                        location = advance_guard(location, &mut map, guard_orientation)
                    }
                    GuardMovementOutcome::Blocked => {
                        guard_orientation = match guard_orientation {
                            GuardOrientation::North => GuardOrientation::East,
                            GuardOrientation::East => GuardOrientation::South,
                            GuardOrientation::South => GuardOrientation::West,
                            GuardOrientation::West => GuardOrientation::North,
                        };
                        location = advance_guard(location, &mut map, guard_orientation);

                        if visited_locations.contains(&(location.0, location.1, guard_orientation))
                        {
                            loop_count += 1;
                            break;
                        }
                    }
                    GuardMovementOutcome::Exit => {
                        // println!("exit {:?}", visited_point);
                        break;
                    }
                }
            }
        }
    }

    println!("{}", loop_count);
}

fn part_one_a() {
    if let Ok(input) = get_challenge_input_as_str(2024, 6) {
        if let Ok(map) = parse_map(&input) {
            println!("{:?}", map);
        } else {
            println!("Unable to parse input");
        }
    }
}

pub fn main() {
    part_one_a();
    // part_two();
}
