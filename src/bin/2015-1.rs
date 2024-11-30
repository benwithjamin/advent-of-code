use advent_of_code::run_on_challenge_input;

#[allow(unused)]
fn part_one(input: &str) -> i32 {
    let mut ret = 0i32;

    input.chars().for_each(|c| {
        match c {
            '(' => ret += 1,
            ')' => ret -= 1,
            _ => {},
        }
    });

    ret
}

#[allow(unused)]
fn part_two(input: &str) -> i32 {
    let mut index = 1i32;
    let mut floor = 0i32;

    for char in input.chars().into_iter() {
        match char {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => {},
        }

        if floor == -1 {
            break
        }

        index += 1
    }

    return index
}

pub fn main() {
    let mut floor = 0i32;
    
    run_on_challenge_input(2015, 1, |line| {
        floor = part_two(line);
    });

    println!("{}", floor);
}