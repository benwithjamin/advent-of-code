use std::{cell::Cell, collections::HashMap};

use advent_of_code::run_on_challenge_input_lines;

enum ParseState {
    OrderingRules,
    UpdatePages,
}

fn is_update_valid(numbers: &Vec<u64>, ordering_rules: &HashMap<u64, Vec<u64>>) -> bool {
    for i in 0..numbers.len() {
        if let Some(rules) = ordering_rules.get(&numbers[i]) {
            if numbers[..i].iter().any(|n| rules.contains(n)) {
                return false;
            }
        }
    }

    true
}

fn part_one() {
    let mut ordering_rules: HashMap<u64, Vec<u64>> = HashMap::new();
    let mut current_state = ParseState::OrderingRules;
    let mut total: u64 = 0;

    run_on_challenge_input_lines(2024, 5, |line| {
        if line.is_empty() {
            current_state = ParseState::UpdatePages;
        } else {
            match current_state {
                ParseState::OrderingRules => {
                    if let &Ok(first_page) = &line[0..2].parse::<u64>() {
                        if let &Ok(second_page) = &line[3..5].parse::<u64>() {
                            ordering_rules
                                .entry(first_page)
                                .or_insert_with(Vec::new)
                                .push(second_page);
                        }
                    }
                }
                ParseState::UpdatePages => {
                    let page_numbers: Vec<u64> = line
                        .split(",")
                        .map(|n| n.parse::<u64>().expect("Error parsing number from string"))
                        .collect();

                    if is_update_valid(&page_numbers, &ordering_rules) {
                        total += page_numbers[page_numbers.len() / 2];
                    }
                }
            }
        }
    });

    println!("part one: {}", total);
}

fn part_two() {
    let mut ordering_rules: HashMap<u64, Vec<u64>> = HashMap::new();
    let mut current_state = ParseState::OrderingRules;
    let mut total: u64 = 0;

    run_on_challenge_input_lines(2024, 5, |line| {
        if line.is_empty() {
            current_state = ParseState::UpdatePages;
        } else {
            match current_state {
                ParseState::OrderingRules => {
                    if let &Ok(first_page) = &line[0..2].parse::<u64>() {
                        if let &Ok(second_page) = &line[3..5].parse::<u64>() {
                            ordering_rules
                                .entry(first_page)
                                .or_insert_with(Vec::new)
                                .push(second_page);
                        }
                    }
                }
                ParseState::UpdatePages => {
                    let mut page_numbers: Vec<u64> = line
                        .split(",")
                        .map(|n| n.parse::<u64>().expect("Error parsing number from string"))
                        .collect();

                    if !is_update_valid(&page_numbers, &ordering_rules) {
                        let page_numbers_slice = &mut page_numbers[..];
                        let slice_of_cells: &[Cell<u64>] =
                            Cell::from_mut(page_numbers_slice).as_slice_of_cells();

                        while !is_update_valid(
                            &slice_of_cells.iter().map(|c| c.get()).collect::<Vec<u64>>(),
                            &ordering_rules,
                        ) {
                            for w in slice_of_cells.windows(2) {
                                if let Some(rules) = ordering_rules.get(&w[1].get()) {
                                    if rules.contains(&w[0].get()) {
                                        Cell::swap(&w[0], &w[1]);
                                    }
                                }
                            }
                        }

                        page_numbers = slice_of_cells.iter().map(|c| c.get()).collect::<Vec<u64>>();
                        total += page_numbers[page_numbers.len() / 2];
                    }
                }
            }
        }
    });

    println!("part two: {}", total);
}

pub fn main() {
    part_one();
    part_two();
}
