use advent_of_code::run_on_challenge_input_lines;

fn part_one(value: impl Into<String>) -> u64 {
    let mut ret: (u64, u64) = (0, 0);

    let string_value: String = value.into();
    ret.0 = string_value
        .chars()
        .find(|c| c.is_numeric())
        .unwrap_or('\0')
        .into();
    ret.1 = string_value
        .chars()
        .rev()
        .find(|c| c.is_numeric())
        .unwrap_or('\0')
        .into();

    ret.0 -= 48;
    ret.1 -= 48;

    (ret.0 * 10) + ret.1
}

pub fn main() {
    let mut calibration_total: u64 = 0;

    run_on_challenge_input_lines(2023, 1, |line| {
        calibration_total += part_one(line);
    });

    println!("{}", calibration_total);
}
