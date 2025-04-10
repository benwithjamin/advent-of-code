use std::collections::HashSet;

use advent_of_code::{get_challenge_input_as_str, maps::Coordinate};

#[allow(unused)]
fn part_one(data: &str, line_limit: usize, memory_space: usize) -> i32 {
    let mut bytes: HashSet<Coordinate<i32>> = HashSet::new();
    let mut lines = data.lines();

    for i in 0..line_limit {
        if let Some(current_line) = lines.next() {
            bytes.insert(current_line.parse().unwrap());
        } else {
            break;
        }
    }

    let mut render_lines: Vec<String> = vec![];

    for y in 0..=memory_space {
        let mut current_line: Vec<char> = vec![];
        for x in 0..=memory_space {
            if bytes.contains(&Coordinate { x: x as i32, y: y as i32 }) {
                current_line.push('#');
            } else {
                current_line.push('.');
            }
        }

        render_lines.push(current_line.iter().collect::<String>());
    }

    println!("{}", render_lines.join("\n"));

    

    0
}

#[allow(unused)]
fn part_two(data: &str) -> i32 {
    0
}

pub fn main() {
    if let Ok(data) = get_challenge_input_as_str(2024, 18) {
        let result = part_one(&data, 1024, 70);
        println!("part one: {}", result);
        let result = part_two(&data);
        println!("part two: {}", result);
    }
}

mod tests {
    #![allow(unused)]
    use super::*;

    const EXAMPLE_INPUT: &str = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";

    #[test]
    fn test_part_one() {
        part_one(EXAMPLE_INPUT, 12, 6);
    }

    #[test]
    fn test_part_two() {}
}
