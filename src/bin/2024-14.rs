use std::collections::{HashMap, HashSet};

use advent_of_code::{get_challenge_input_as_str, maps::Coordinate};
use bmp::Pixel;
use regex::Regex;

#[derive(Debug)]
struct Robot {
    position: Coordinate<i32>,
    velocity: Coordinate<i32>,
}

impl Robot {
    pub fn update_location(&mut self, map_width: i32, map_height: i32) -> Coordinate<i32> {
        self.position.x = (self.position.x + self.velocity.x).rem_euclid(map_width);
        self.position.y = (self.position.y + self.velocity.y).rem_euclid(map_height);

        self.position.clone()
    }
}

struct BathroomData {
    width: i32,
    height: i32,
    robots: Vec<Robot>,
}

impl BathroomData {
    pub fn render(&self, index: i32) {
        use bmp::Image;
        let mut image = Image::new(self.width as u32, self.height as u32);

        let mut robot_positions: HashMap<Coordinate<i32>, usize> = HashMap::new();
        self.robots
            .iter()
            .for_each(|robot| *robot_positions.entry(robot.position).or_insert(0) += 1);

        for x in 0..self.width {
            for y in 0..self.height {
                if let Some(_) = robot_positions.get(&Coordinate { x, y }) {
                    image.set_pixel(x as u32, y as u32, Pixel::new(255, 255, 255));
                }
            }
        }

        let _ = image.save(format!("output/{}.bmp", index));
    }

    pub fn print(&self) {
        let mut robot_positions: HashMap<Coordinate<i32>, usize> = HashMap::new();
        self.robots
            .iter()
            .for_each(|robot| *robot_positions.entry(robot.position).or_insert(0) += 1);

        let mut current_line: Vec<char> = Vec::with_capacity(self.width as usize);

        for x in 0..self.width {
            for y in 0..self.height {
                if let Some(&robots) = robot_positions.get(&Coordinate { x, y }) {
                    current_line.push(char::from_digit(robots as u32, 10).unwrap());
                } else {
                    current_line.push('.');
                }
            }
            println!("{}", current_line.clone().into_iter().collect::<String>());
            current_line.clear();
        }
    }
}

fn parse_data(data: &str, width: i32, height: i32) -> BathroomData {
    let robot_regex: Regex = Regex::new(r"p=([0-9]+),([0-9]+) v=([\-0-9]+),([\-0-9]+)").unwrap();

    let robots: Vec<Robot> = data
        .lines()
        .map(|line| {
            let captures = robot_regex.captures(line).unwrap();

            return Robot {
                position: Coordinate {
                    x: captures.get(1).unwrap().as_str().parse::<i32>().unwrap(),
                    y: captures.get(2).unwrap().as_str().parse::<i32>().unwrap(),
                },
                velocity: Coordinate {
                    x: captures.get(3).unwrap().as_str().parse::<i32>().unwrap(),
                    y: captures.get(4).unwrap().as_str().parse::<i32>().unwrap(),
                },
            };
        })
        .collect();

    BathroomData {
        width,
        height,
        robots,
    }
}

fn update_robot_locations(data: &mut BathroomData, steps: i32) {
    for i in 0..steps {
        for robot in data.robots.iter_mut() {
            robot.update_location(data.width, data.height);
        }

        data.render(i);
    }
}

fn calculate_safety_factor(data: &BathroomData) -> i32 {
    let mut quadrant_a: i32 = 0;
    let mut quadrant_b: i32 = 0;
    let mut quadrant_c: i32 = 0;
    let mut quadrant_d: i32 = 0;

    let midpoint_x = (data.width - 1) / 2;
    let midpoint_y = (data.height - 1) / 2;

    data.robots.iter().for_each(|robot| match robot.position {
        p if p.x == midpoint_x || p.y == midpoint_y => {}
        p if p.x < midpoint_x && p.y < midpoint_y => quadrant_a += 1,
        p if p.x > midpoint_x && p.y < midpoint_y => quadrant_b += 1,
        p if p.x < midpoint_x && p.y > midpoint_y => quadrant_c += 1,
        p if p.x > midpoint_x && p.y > midpoint_y => quadrant_d += 1,
        _ => unreachable!(),
    });

    println!("a {quadrant_a} b {quadrant_b} c {quadrant_c} d {quadrant_d}");

    quadrant_a * quadrant_b * quadrant_c * quadrant_d
}

#[allow(unused)]
fn part_one(data: &mut BathroomData) -> i32 {
    update_robot_locations(data, 100);
    calculate_safety_factor(data)
}

#[allow(unused)]
fn part_two(data: &mut BathroomData) -> i32 {
    let mut seconds = 0;

    loop {
        let mut occupied = HashSet::new();
        for mut robot in data.robots.iter_mut() {
            occupied.insert(robot.update_location(data.width, data.height));
        }

        let connected_count = occupied
            .iter()
            .filter(|c| c.neighbours().iter().any(|n| occupied.contains(n)))
            .count();
        if connected_count >= data.robots.len() / 2 {
            return seconds;
        }

        seconds += 1;
    }
}

pub fn main() {
    if let Ok(data) = get_challenge_input_as_str(2024, 14) {
        let mut data = parse_data(&data, 101, 103);

        // let result = part_one(&mut data);
        // println!("part one: {}", result);
        let result = part_two(&mut data);
        println!("part two: {}", result);
    }
}

mod tests {
    #![allow(unused)]
    use super::*;

    const EXAMPLE_INPUT: &str = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

    const SINGLE_ROBOT: &str = "p=2,4 v=2,-3";

    #[test]
    fn test_parse() {
        let data = parse_data(EXAMPLE_INPUT, 11, 7);

        assert_eq!(data.robots.len(), 12);
        assert_eq!(data.width, 11);
        assert_eq!(data.height, 7);
    }

    #[test]
    fn test_part_one() {
        let mut data = parse_data(EXAMPLE_INPUT, 11, 7);
        update_robot_locations(&mut data, 100);
        let safety_factor = calculate_safety_factor(&data);

        assert_eq!(safety_factor, 12);
    }

    #[test]
    fn test_part_two() {}
}
