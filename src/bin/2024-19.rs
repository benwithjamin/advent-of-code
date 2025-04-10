use advent_of_code::get_challenge_input_as_str;

fn get_matching_chunks(input: &str, chunks: &Vec<String>) -> Vec<String> {
    let mut matching_chunks: Vec<String> = chunks
        .iter()
        .filter(|&a| input.starts_with(a))
        .map(|c| c.to_string())
        .collect();

    matching_chunks.sort_by(|a, b| b.len().cmp(&a.len()));

    matching_chunks
}

fn is_design_possible(design: &str, available: &Vec<String>) -> bool {
    let matching_chunks = get_matching_chunks(design, available);

    matching_chunks.iter().any(|chunk| {
        let remaining = design.strip_prefix(chunk).unwrap();

        if remaining.is_empty() {
            return true;
        } else if !get_matching_chunks(remaining, available).is_empty() {
            return is_design_possible(remaining, available);
        } else {
            return false;
        }
    })
}

fn test(design: &str, available: &Vec<String>) -> i64 {
    let matching_chunks = get_matching_chunks(design, available);

    matching_chunks
        .iter()
        .map(|chunk| {
            let remaining = design.strip_prefix(chunk).unwrap();

            if remaining.is_empty() {
                return true;
            } else if !get_matching_chunks(remaining, available).is_empty() {
                return is_design_possible(remaining, available);
            } else {
                return false;
            }
        })
        .filter(|&x| x)
        .count() as i64
}

fn get_matching_combinations(
    design: &str,
    available: &Vec<String>,
    current_match: &mut Vec<String>,
    all_matches: &mut i32,
) {
    if design.is_empty() {
        // all_matches.push(current_match.to_vec());
        println!("{:?}", current_match);
        *all_matches += 1;
        return;
    }

    for chunk in available.iter() {
        if design.starts_with(chunk) {
            current_match.push(chunk.to_string());
            get_matching_combinations(
                &design[chunk.len()..],
                available,
                current_match,
                all_matches,
            );
            current_match.pop();
        }
    }
}

/**
 * Start with input
 * For each
 */

#[allow(unused)]
fn part_one(data: &str) -> i32 {
    let mut possible_designs: i32 = 0;
    let mut lines = data.lines();
    let available_patterns: Vec<String> = lines
        .next()
        .unwrap()
        .split(",")
        .map(|s| s.trim().to_string())
        .collect();

    for line in lines {
        if line.is_empty() {
            continue;
        }

        if is_design_possible(line, &available_patterns) {
            possible_designs += 1;
        }
    }

    possible_designs
}

#[allow(unused)]
fn part_two(data: &str) -> i64 {
    let mut possible_designs: i64 = 0;
    let mut lines = data.lines();
    let available_patterns: Vec<String> = lines
        .next()
        .unwrap()
        .split(",")
        .map(|s| s.trim().to_string())
        .collect();

    for line in lines {
        if line.is_empty() {
            continue;
        }

        let mut combinations: i32 = 0;
        // get_matching_combinations(line, &available_patterns, &mut vec![], &mut combinations);
        // println!("{line}: {combinations}");

        possible_designs += test(line, &available_patterns);
    }

    possible_designs
}

pub fn main() {
    if let Ok(data) = get_challenge_input_as_str(2024, 19) {
        let result = part_one(&data);
        println!("part one: {}", result);
        let result = part_two(&data);
        println!("part two: {}", result);
    }
}

mod tests {
    #![allow(unused)]
    use super::*;

    const EXAMPLE_INPUT: &str = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(EXAMPLE_INPUT), 6);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(EXAMPLE_INPUT), 16)
    }
}
