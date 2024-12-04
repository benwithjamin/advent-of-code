use advent_of_code::run_on_challenge_input_lines;

fn are_levels_safe(levels: Vec<i64>) -> bool {
    match levels[0] - levels[levels.len() - 1] {
        difference if difference > 0 => {
            return levels.windows(2).all(|l| (l[0] > l[1]) && l[0] - l[1] <= 3)
        }
        difference if difference < 0 => {
            return levels.windows(2).all(|l| l[0] < l[1] && l[1] - l[0] <= 3)
        }
        difference if difference == 0 => return false,
        _ => unreachable!(),
    }
}

fn is_report_safe(line: &str) -> bool {
    let levels: Vec<i64> = line
        .split(" ")
        .map(|level| level.parse::<i64>().unwrap())
        .collect();

    are_levels_safe(levels)
}

fn is_report_safe_with_problem_dampener(line: &str) -> bool {
    let levels: Vec<i64> = line
        .split(" ")
        .map(|level| level.parse::<i64>().unwrap())
        .collect();

    for i in 0..levels.len() {
        let mut adjusted_levels = levels.clone();
        adjusted_levels.remove(i);

        if are_levels_safe(adjusted_levels) {
            return true;
        }
    }

    false
}

fn part_one() {
    let mut safe_levels: u64 = 0;

    run_on_challenge_input_lines(2024, 2, |line| {
        if is_report_safe(&line) {
            safe_levels += 1;
        }
    });

    println!("part one: {}", safe_levels);
}

fn part_two() {
    let mut safe_levels: u64 = 0;

    run_on_challenge_input_lines(2024, 2, |line| {
        if is_report_safe_with_problem_dampener(&line) {
            safe_levels += 1;
        }
    });

    println!("part two: {}", safe_levels);
}

pub fn main() {
    part_one();
    part_two();
}
