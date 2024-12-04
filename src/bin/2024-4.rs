use advent_of_code::{run_on_challenge_input_lines, run_on_challenge_input_lines_ttb};
use regex::Regex;

fn run_on_diagonals<F>(year: u64, day: u64, flip: bool, mut func: F)
where
    F: FnMut(usize, &str),
{
    let mut line_index: usize = 0;
    let mut diagonals: Vec<Vec<char>> = vec![vec![]];

    run_on_challenge_input_lines(year, day, |l| {
        let line: String = if flip {
            l.chars().rev().collect()
        } else {
            l.to_string()
        };

        for (index, character) in line.char_indices() {
            if let None = diagonals.get(index + line_index) {
                diagonals.push(vec![character]);
            } else {
                diagonals[index + line_index].push(character);
            }
        }

        line_index += 1;
    });

    let mut current_line = 0;
    for diagonal_line in diagonals.iter() {
        let diagonal_line_str = String::from_iter(diagonal_line.into_iter());

        func(current_line, &diagonal_line_str);

        current_line += 1;
    }
}

fn part_one() {
    let mut match_count: usize = 0;

    /* Horizontal */
    run_on_challenge_input_lines(2024, 4, |line| {
        match_count += line.matches("XMAS").count();
        match_count += line.matches("SAMX").count();
    });

    /* Vertical */
    run_on_challenge_input_lines_ttb(2024, 4, |line| {
        match_count += line.matches("XMAS").count();
        match_count += line.matches("SAMX").count();
    });

    /* Diagonal LTR */
    run_on_diagonals(2024, 4, false, |_, line| {
        match_count += line.matches("XMAS").count();
        match_count += line.matches("SAMX").count();
    });

    /* Diagonal RTL */
    run_on_diagonals(2024, 4, true, |_, line| {
        match_count += line.matches("XMAS").count();
        match_count += line.matches("SAMX").count();
    });

    println!("part one: {}", match_count);
}

fn part_two() {
    let re_ltr = Regex::new("MAS").unwrap();
    let re_rtl = Regex::new("SAM").unwrap();

    let mut a_locations: Vec<(usize, usize)> = vec![];
    let mut grid: Vec<Vec<char>> = vec![];
    let mut matches = 0;

    run_on_challenge_input_lines(2024, 4, |line| {
        let line_characters: Vec<char> = line.chars().collect();
        grid.push(line_characters);
    });

    for row in 0..grid.len() {
        for column in 0..grid[0].len() {
            if grid[row][column] == 'A' {
                a_locations.push((row, column));
            }
        }
    }

    for &(row, column) in a_locations.iter() {
        if row == 0 || column == 0 || row == grid.len() - 1 || column == grid[row].len() - 1 {
            continue;
        }

        let diagonal_ltr: String = vec![
            grid[row - 1][column - 1],
            grid[row][column],
            grid[row + 1][column + 1],
        ]
        .into_iter()
        .collect();

        let diagonal_rtl: String = vec![
            grid[row - 1][column + 1],
            grid[row][column],
            grid[row + 1][column - 1],
        ]
        .into_iter()
        .collect();

        if (diagonal_ltr == "MAS" || diagonal_ltr == "SAM")
            && (diagonal_rtl == "MAS" || diagonal_rtl == "SAM")
        {
            matches += 1;
        }
    }

    println!("part two: {}", matches);
}

pub fn main() {
    part_one();
    part_two();
}
