use std::{
    collections::HashSet,
    fmt::{Debug, Display},
};

use advent_of_code::{
    get_challenge_input_as_str,
    maps::{Coordinate, Direction, Direction::*},
};

struct WarehouseData {
    width: usize,
    height: usize,
    robot_location: Coordinate<i32>,
    boxes: HashSet<Coordinate<i32>>,
    walls: HashSet<Coordinate<i32>>,
    robot_moves: Vec<Direction>,
}

impl Debug for WarehouseData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut lines: Vec<String> = Vec::new();

        for y in 0..self.height {
            let mut x = 0;
            let mut line: Vec<char> = Vec::new();

            while x < self.width {
                let coordinate = Coordinate {
                    x: x as i32,
                    y: y as i32,
                };
                match coordinate {
                    _ if self.walls.contains(&coordinate) => line.push('#'),
                    _ if self.boxes.contains(&coordinate) => {
                        line.push('[');
                        line.push(']');
                        x += 1;
                    }
                    _ if self.robot_location == coordinate => line.push('@'),
                    _ => line.push('.'),
                }

                x += 1
            }

            lines.push(line.iter().collect::<String>());
        }

        write!(f, "{}", lines.join("\n"))
    }
}

impl Display for WarehouseData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut lines: Vec<String> = Vec::new();

        for y in 0..self.height {
            let mut line: Vec<char> = Vec::new();

            for x in 0..self.width {
                let coordinate = Coordinate {
                    x: x as i32,
                    y: y as i32,
                };
                match coordinate {
                    _ if self.walls.contains(&coordinate) => line.push('#'),
                    _ if self.boxes.contains(&coordinate) => line.push('O'),
                    _ if self.robot_location == coordinate => line.push('@'),
                    _ => line.push('.'),
                }
            }

            lines.push(line.iter().collect::<String>());
        }

        write!(f, "{}", lines.join("\n"))
    }
}

fn parse_data(data: &str, part_two: bool) -> WarehouseData {
    let mut robot_location: Coordinate<i32> = Coordinate { x: 0, y: 0 };
    let mut boxes: HashSet<Coordinate<i32>> = HashSet::new();
    let mut walls: HashSet<Coordinate<i32>> = HashSet::new();
    let mut robot_moves: Vec<Direction> = vec![];
    let mut width: usize = 0;
    let mut height: usize = 0;

    data.lines().enumerate().for_each(|(row, line)| {
        let mut line = line.to_string();

        if line.is_empty() {
            return;
        };

        if line.starts_with('#') {
            if part_two {
                line = line
                    .split("")
                    .map(|chunk| match chunk {
                        "@" => "@.".to_string(),
                        "O" => "[]".to_string(),
                        _ => chunk.repeat(2),
                    })
                    .collect::<String>();
            }

            width = line.len();
            height += 1;

            for (column, character) in line.chars().enumerate() {
                match character {
                    '#' => {
                        walls.insert(Coordinate {
                            x: column as i32,
                            y: row as i32,
                        });
                    }
                    'O' | '[' => {
                        boxes.insert(Coordinate {
                            x: column as i32,
                            y: row as i32,
                        });
                    }
                    '@' => {
                        robot_location = Coordinate {
                            x: column as i32,
                            y: row as i32,
                        };
                    }
                    _ => {}
                };
            }
        }

        robot_moves.extend(
            line.chars()
                .map(|c| match c {
                    '<' => Some(West),
                    '>' => Some(East),
                    '^' => Some(North),
                    'v' => Some(South),
                    _ => None,
                })
                .filter(Option::is_some)
                .map(Option::unwrap)
                .collect::<Vec<Direction>>(),
        );
    });

    WarehouseData {
        width,
        height,
        robot_location,
        boxes,
        walls,
        robot_moves,
    }
}

fn can_move_box(
    from: Coordinate<i32>,
    direction: Direction,
    data: &mut WarehouseData,
    part_two: bool,
) -> bool {
    match from.neighbour(direction) {
        coordinate if data.walls.contains(&coordinate) => {
            return false;
        }
        coordinate if part_two && direction == West => {
            if data.boxes.contains(&coordinate.neighbour(West)) {
                if can_move_box(coordinate.neighbour(West), direction, data, part_two) {
                    data.boxes.remove(&coordinate.neighbour(West));
                    data.boxes
                        .insert(coordinate.neighbour(West).neighbour(West));

                    return true;
                }
                return false;
            }

            return true;
        }
        coordinate if part_two && direction == North => {
            if data.boxes.contains(&coordinate.neighbour(NorthWest)) {
                if can_move_box(coordinate.neighbour(NorthWest), direction, data, part_two) {
                    data.boxes.remove(&coordinate.neighbour(NorthWest));
                    data.boxes
                        .insert(coordinate.neighbour(NorthWest).neighbour(North));
                } else {
                    return false;
                }
            }
            if data.boxes.contains(&coordinate.neighbour(North)) {
                if can_move_box(coordinate.neighbour(North), direction, data, part_two) {
                    data.boxes.remove(&coordinate.neighbour(North));
                    data.boxes
                        .insert(coordinate.neighbour(North).neighbour(North));
                } else {
                    return false;
                }
            }
            if data.boxes.contains(&coordinate.neighbour(NorthEast)) {
                if can_move_box(coordinate.neighbour(NorthEast), direction, data, part_two) {
                    data.boxes.remove(&coordinate.neighbour(NorthEast));
                    data.boxes
                        .insert(coordinate.neighbour(NorthEast).neighbour(North));
                } else {
                    return false;
                }
            }
            return true;
        }
        coordinate if data.boxes.contains(&coordinate) => {
            if can_move_box(coordinate, direction, data, part_two) {
                data.boxes.remove(&coordinate);
                data.boxes.insert(
                    coordinate
                        + match direction {
                            North => Coordinate { x: 0, y: -1 },
                            East => Coordinate { x: 1, y: 0 },
                            South => Coordinate { x: 0, y: 1 },
                            West => Coordinate { x: -1, y: 0 },
                            _ => unreachable!(),
                        },
                );
                return true;
            }

            return false;
        }
        _ => {
            return true;
        }
    }
}

fn can_make_move(
    from: Coordinate<i32>,
    direction: Direction,
    data: &mut WarehouseData,
    part_two: bool,
) -> bool {
    match from.neighbour(direction) {
        coordinate if data.walls.contains(&coordinate) => {
            return false;
        }
        coordinate if part_two && direction == West => {
            if data.boxes.contains(&coordinate.neighbour(West)) {
                if can_move_box(coordinate.neighbour(West), direction, data, part_two) {
                    data.boxes.remove(&coordinate.neighbour(West));
                    data.boxes
                        .insert(coordinate.neighbour(West).neighbour(West));

                    return true;
                }
                return false;
            }

            return true;
        }
        coordinate if part_two && direction == North => {
            if data.boxes.contains(&coordinate.neighbour(NorthWest)) {
                if can_move_box(coordinate.neighbour(NorthWest), direction, data, part_two) {
                    data.boxes.remove(&coordinate.neighbour(NorthWest));
                    data.boxes
                        .insert(coordinate.neighbour(NorthWest).neighbour(North));
                } else {
                    return false;
                }
            }

            if data.boxes.contains(&coordinate.neighbour(North)) {
                if can_move_box(coordinate.neighbour(North), direction, data, part_two) {
                    data.boxes.remove(&coordinate.neighbour(North));
                    data.boxes
                        .insert(coordinate.neighbour(North).neighbour(North));
                } else {
                    return false;
                }
            }

            return true;
        }
        coordinate if data.boxes.contains(&coordinate) => {
            if can_move_box(coordinate, direction, data, part_two) {
                data.boxes.remove(&coordinate);
                data.boxes.insert(
                    coordinate
                        + match direction {
                            North => Coordinate { x: 0, y: -1 },
                            East => Coordinate { x: 1, y: 0 },
                            South => Coordinate { x: 0, y: 1 },
                            West => Coordinate { x: -1, y: 0 },
                            _ => unreachable!(),
                        },
                );
                return true;
            }

            return false;
        }
        _ => {
            return true;
        }
    }
}

fn solve(data: &mut WarehouseData, part_two: bool) -> i32 {
    for (index, m) in data.robot_moves.clone().iter().enumerate() {
        if can_make_move(data.robot_location, *m, data, part_two) {
            data.robot_location += match m {
                North => Coordinate { x: 0, y: -1 },
                East => Coordinate { x: 1, y: 0 },
                South => Coordinate { x: 0, y: 1 },
                West => Coordinate { x: -1, y: 0 },
                _ => unreachable!(),
            }
        }

        if part_two {
            println!("move {}: {:?}\n{:?}\n\n", index + 1, m, data);
        }
    }

    data.boxes.iter().map(|b| (b.y * 100 + b.x) as i32).sum()
}

#[allow(unused)]
fn part_one(data: &mut WarehouseData) -> i32 {
    solve(data, false)
}

#[allow(unused)]
fn part_two(data: &mut WarehouseData) -> i32 {
    solve(data, true)
}

pub fn main() {
    if let Ok(data) = get_challenge_input_as_str(2024, 15) {
        let mut data = parse_data(&data, false);
        let result = part_one(&mut data);
        println!("part one: {}", result);
    }
}

mod tests {
    #![allow(unused)]
    use super::*;

    const EXAMPLE_INPUT: &str = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";

    const LARGER_EXAMPLE: &str = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

    const PART_TWO_EXAMPLE: &str = "#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^";

    #[test]
    fn test_parse_part_one() {
        let data = parse_data(EXAMPLE_INPUT, false);

        assert_eq!(data.boxes.len(), 6);
        assert_eq!(data.robot_location, Coordinate { x: 2, y: 2 });
        assert_eq!(data.robot_moves.len(), 15);
    }

    #[test]
    fn test_parse_part_two() {
        let data = parse_data(LARGER_EXAMPLE, true);

        println!("{}", data);
    }

    #[test]
    fn test_part_one() {
        let mut data = parse_data(EXAMPLE_INPUT, false);

        let result = part_one(&mut data);

        assert_eq!(result, 2028);

        let mut data = parse_data(LARGER_EXAMPLE, false);

        let result = part_one(&mut data);

        assert_eq!(result, 10092);
    }

    #[test]
    fn test_part_two() {
        let mut data = parse_data(PART_TWO_EXAMPLE, true);

        let result = part_two(&mut data);

        assert_eq!(result, 9021);
    }
}
