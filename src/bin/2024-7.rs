use advent_of_code::get_challenge_input_as_str;

#[derive(Debug)]
struct CalibrationEquation {
    test_value: u64,
    input_numbers: Vec<u64>,
}

#[derive(Clone, Debug)]
enum Operator {
    Plus,
    Multiply,
    Concatenate,
}

fn parse_input(data: &str) -> Result<Vec<CalibrationEquation>, String> {
    let mut equations: Vec<CalibrationEquation> = vec![];

    for line in data.lines() {
        let (test_value, inputs) =
            line.split_at(line.find(':').ok_or_else(|| "malformed line".to_string())?);

        equations.push(CalibrationEquation {
            test_value: test_value.parse::<u64>().unwrap_or(0),
            input_numbers: inputs
                .split(' ')
                .map(|i| i.parse::<u64>().unwrap_or(0))
                .filter(|&i| i != 0)
                .collect(),
        })
    }

    Ok(equations)
}

fn plus_multiply_combinations(length: usize) -> Vec<Vec<Operator>> {
    if length == 0 {
        return vec![vec![]];
    }

    let shorter_combinations = plus_multiply_combinations(length - 1);
    let mut combinations = Vec::new();

    for combination in shorter_combinations {
        let mut plus_comb = combination.clone();
        plus_comb.push(Operator::Plus);
        combinations.push(plus_comb);

        let mut multiply_comb = combination.clone();
        multiply_comb.push(Operator::Multiply);
        combinations.push(multiply_comb);
    }

    combinations
}

fn plus_multiply_concatenate_combinations(length: usize) -> Vec<Vec<Operator>> {
    if length == 0 {
        return vec![vec![]];
    }

    let shorter_combinations = plus_multiply_concatenate_combinations(length - 1);
    let mut combinations = Vec::new();

    for combination in shorter_combinations {
        let mut plus_comb = combination.clone();
        plus_comb.push(Operator::Plus);
        combinations.push(plus_comb);

        let mut multiply_comb = combination.clone();
        multiply_comb.push(Operator::Multiply);
        combinations.push(multiply_comb);

        let mut concat_comb = combination.clone();
        concat_comb.push(Operator::Concatenate);
        combinations.push(concat_comb);
    }

    combinations
}

fn can_solve(equation: &CalibrationEquation) -> bool {
    for operator_combination in plus_multiply_combinations(equation.input_numbers.len()) {
        let mut total: u64 = equation.input_numbers[0];

        for i in 1..equation.input_numbers.len() {
            match operator_combination[i] {
                Operator::Plus => total += equation.input_numbers[i],
                Operator::Multiply => total *= equation.input_numbers[i],
                _ => {}
            }
        }

        if total == equation.test_value {
            return true;
        }
    }

    false
}

fn can_solve_with_concatenation(equation: &CalibrationEquation) -> bool {
    for operator_combination in plus_multiply_concatenate_combinations(equation.input_numbers.len())
    {
        let mut total: u64 = equation.input_numbers[0];

        for i in 1..equation.input_numbers.len() {
            if total > equation.test_value {
                break;
            }
            match operator_combination[i] {
                Operator::Plus => total += equation.input_numbers[i],
                Operator::Multiply => total *= equation.input_numbers[i],
                Operator::Concatenate => {
                    total = format!("{}{}", total, equation.input_numbers[i])
                        .parse::<u64>()
                        .unwrap()
                }
            }
        }

        if total == equation.test_value {
            return true;
        }
    }

    false
}

#[allow(unused)]
fn part_one(equations: &Vec<CalibrationEquation>) -> u64 {
    let mut solvable: u64 = 0;

    for equation in equations {
        if can_solve(&equation) {
            solvable += equation.test_value;
        }
    }

    solvable
}

#[allow(unused)]
fn part_two(equations: &Vec<CalibrationEquation>) -> u64 {
    let mut solvable: u64 = 0;

    for equation in equations {
        if can_solve_with_concatenation(&equation) {
            solvable += equation.test_value;
        }
    }

    solvable
}

pub fn main() {
    if let Ok(data) = get_challenge_input_as_str(2024, 7) {
        if let Ok(equations) = parse_input(&data) {
            let part_one_result = part_one(&equations);

            println!("part one: {}", part_one_result);

            let part_two_result = part_two(&equations);

            println!("part two: {}", part_two_result);
        }
    }
}

mod tests {
    #![allow(unused)]
    use super::*;

    const EXAMPLE_INPUT: &str = r"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn test_parse_input() {
        let equations = parse_input(EXAMPLE_INPUT);

        assert!(equations.is_ok_and(|e| e.len() == 9));
    }

    #[test]
    fn test_part_one() {
        if let Ok(equations) = parse_input(EXAMPLE_INPUT) {
            let result = part_one(&equations);

            assert_eq!(result, 3749);
        }
    }

    #[test]
    fn test_part_two() {
        if let Ok(equations) = parse_input(EXAMPLE_INPUT) {
            let result = part_two(&equations);

            assert_eq!(result, 11387);
        }
    }
}
