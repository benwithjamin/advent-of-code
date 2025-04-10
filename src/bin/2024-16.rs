use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    fmt::Display,
};

use advent_of_code::{
    get_challenge_input_as_str,
    maps::{Coordinate, Direction},
};

struct Maze {
    width: i32,
    height: i32,
    start: Coordinate<i32>,
    end: Coordinate<i32>,
    walls: HashSet<Coordinate<i32>>,
}

impl Display for Maze {
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
                    _ if self.start == coordinate => line.push('S'),
                    _ if self.end == coordinate => line.push('E'),
                    _ => line.push('.'),
                }

                x += 1
            }

            lines.push(line.iter().collect::<String>());
        }

        write!(f, "{}", lines.join("\n"))
    }
}

fn parse_data(data: &str) -> Result<Maze, String> {
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

    let mut start: Coordinate<i32> = Coordinate { x: 0, y: 0 };
    let mut end: Coordinate<i32> = Coordinate { x: 0, y: 0 };
    let mut walls: HashSet<Coordinate<i32>> = HashSet::new();

    for (y, line) in data.lines().enumerate() {
        for (x, character) in line.chars().enumerate() {
            match character {
                '#' => {
                    walls.insert(Coordinate {
                        x: x as i32,
                        y: y as i32,
                    });
                }
                'S' => {
                    start = Coordinate {
                        x: x as i32,
                        y: y as i32,
                    };
                }
                'E' => {
                    end = Coordinate {
                        x: x as i32,
                        y: y as i32,
                    };
                }
                _ => {}
            }
        }
    }

    Ok(Maze {
        width,
        height,
        start,
        end,
        walls,
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct MazeNode {
    coordinate: Coordinate<i32>,
    cost: i32,
    direction: Direction,
}

impl Ord for MazeNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for MazeNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn solve(maze: Maze) -> i32 {
    use Direction::*;

    let mut frontier: BinaryHeap<MazeNode> = BinaryHeap::new();
    frontier.push(MazeNode {
        coordinate: maze.start,
        cost: 0,
        direction: East,
    });

    let mut came_from: HashMap<Coordinate<i32>, Option<Coordinate<i32>>> = HashMap::new();
    let mut cost_so_far: HashMap<Coordinate<i32>, i32> = HashMap::new();

    came_from.insert(maze.start, None);
    cost_so_far.insert(maze.start, 0);

    let mut score: i32 = std::i32::MAX;

    while !frontier.is_empty() {
        let current = frontier.pop().unwrap();

        if current.coordinate == maze.end {
            let current_score = *cost_so_far.get(&current.coordinate).unwrap();

            score = if score > current_score {
                current_score
            } else {
                score
            };
        }

        for &neighbour in current.coordinate.neighbours().iter() {
            if maze.walls.contains(&neighbour) {
                continue;
            }

            let new_direction = current.coordinate.direction_to(&neighbour).unwrap();

            let new_cost = cost_so_far.get(&current.coordinate).unwrap_or(&0)
                + if new_direction != current.direction {
                    1001
                } else {
                    1
                };

            if !cost_so_far.contains_key(&neighbour)
                || new_cost < *cost_so_far.get(&neighbour).unwrap()
            {
                cost_so_far.insert(neighbour, new_cost);

                frontier.push(MazeNode {
                    coordinate: neighbour,
                    cost: new_cost,
                    direction: new_direction,
                });

                came_from.insert(neighbour, Some(current.coordinate));
            }
        }
    }

    score
}

#[allow(unused)]
fn part_one(maze: Maze) -> i32 {
    solve(maze)
}

#[allow(unused)]
fn part_two(data: &str) -> i32 {
    0
}

pub fn main() {
    if let Ok(data) = get_challenge_input_as_str(2024, 16) {
        if let Ok(data) = parse_data(&data) {
            let result = part_one(data);
            println!("part one: {}", result);
            // let result = part_two(&data);
            // println!("part two: {}", result);
        }
    }
}

mod tests {
    #![allow(unused)]
    use super::*;

    const EXAMPLE_INPUT: &str = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";

    const SECOND_EXAMPLE: &str = "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";

    const ALTERNATE: &str = "###########################
#######################..E#
######################..#.#
#####################..##.#
####################..###.#
###################..##...#
##################..###.###
#################..####...#
################..#######.#
###############..##.......#
##############..###.#######
#############..####.......#
############..###########.#
###########..##...........#
##########..###.###########
#########..####...........#
########..###############.#
#######..##...............#
######..###.###############
#####..####...............#
####..###################.#
###..##...................#
##..###.###################
#..####...................#
#.#######################.#
#S........................#
###########################";

    const LONGER_ALTERNATE: &str = "##########################################################################################################
#.........#.........#.........#.........#.........#.........#.........#.........#.........#.........#...E#
#.........#.........#.........#.........#.........#.........#.........#.........#.........#.........#....#
#....#....#....#....#....#....#....#....#....#....#....#....#....#....#....#....#....#....#....#....#....#
#....#....#....#....#....#....#....#....#....#....#....#....#....#....#....#....#....#....#....#....#....#
#....#....#....#....#....#....#....#....#....#....#....#....#....#....#....#....#....#....#....#....#....#
#....#....#....#....#....#....#....#....#....#....#....#....#....#....#....#....#....#....#....#....#....#
#....#.........#.........#.........#.........#.........#.........#.........#.........#.........#.........#
#S...#.........#.........#.........#.........#.........#.........#.........#.........#.........#.........#
##########################################################################################################";

    const ANOTHER_ALTERNATE: &str = "##########
#.......E#
#.##.#####
#..#.....#
##.#####.#
#S.......#
##########";

    #[test]
    fn test_parse() {
        let data = parse_data(EXAMPLE_INPUT);

        assert!(data.is_ok());
        assert_eq!(data.as_ref().unwrap().height, 15);
        assert_eq!(data.as_ref().unwrap().width, 15);
    }

    #[test]
    fn test_part_one() {
        if let Ok(data) = parse_data(EXAMPLE_INPUT) {
            assert_eq!(solve(data), 7036);
        }

        if let Ok(data) = parse_data(SECOND_EXAMPLE) {
            assert_eq!(solve(data), 11048);
        }

        if let Ok(data) = parse_data(ALTERNATE) {
            assert_eq!(solve(data), 21148);
        }

        if let Ok(data) = parse_data(LONGER_ALTERNATE) {
            assert_eq!(solve(data), 41210);
        }

        if let Ok(data) = parse_data(ANOTHER_ALTERNATE) {
            assert_eq!(solve(data), 4013);
        }
    }

    #[test]
    fn test_part_two() {}
}
