use std::collections::{HashMap, HashSet};

use advent_of_code::{
    get_challenge_input_as_str,
    maps::{Coordinate, Direction, MapData},
};

type PlotData = MapData<char>;

fn count_fences(plot: char, coordinate: &Coordinate<usize>, map: &PlotData) -> i64 {
    let adjacent_plots = map.get_valid_adjacent_coordinates(coordinate);

    return 4 - adjacent_plots
        .iter()
        .filter(|coordinate| unsafe { map.unchecked_get(coordinate) } == plot)
        .count() as i64;
}

/**
 * while there is an adjacent plot
 * add number of fences to total
 */

fn measure_plot_perimeter(
    plot: char,
    coordinate: &Coordinate<usize>,
    map: &PlotData,
    checked_plots: &mut HashSet<Coordinate<usize>>,
    perimeter: &mut i64,
    area: &mut i64,
) {
    if checked_plots.contains(coordinate) {
        return;
    }

    checked_plots.insert(*coordinate);

    *area += 1;
    *perimeter += count_fences(plot, coordinate, map);

    let adjacent_plots = map.get_valid_adjacent_coordinates(coordinate);

    adjacent_plots.iter().for_each(|adjacent_plot| unsafe {
        if map.unchecked_get(adjacent_plot) == plot {
            measure_plot_perimeter(plot, adjacent_plot, map, checked_plots, perimeter, area);
        }
    });
}

fn measure_plot_sides(
    plot: char,
    coordinate: &Coordinate<usize>,
    map: &PlotData,
    checked_plots: &mut HashSet<Coordinate<usize>>,
    current_plot: &mut HashSet<Coordinate<usize>>,
) {
    if checked_plots.contains(coordinate) {
        return;
    }

    checked_plots.insert(*coordinate);
    current_plot.insert(*coordinate);

    let adjacent_plots = map.get_valid_adjacent_coordinates(coordinate);

    adjacent_plots.iter().for_each(|adjacent_plot| unsafe {
        if !checked_plots.contains(adjacent_plot) {
            if map.unchecked_get(adjacent_plot) == plot {
                measure_plot_sides(plot, adjacent_plot, map, checked_plots, current_plot);
            }
        }
    });
}

fn part_one(data: &str) -> i64 {
    let mut checked_plots: HashSet<Coordinate<usize>> = HashSet::new();
    let mut fencing_price: i64 = 0;

    match PlotData::new_from_str(data) {
        Ok(plot_data) => {
            for (coordinate, &plot) in plot_data.enumerate() {
                if !checked_plots.contains(&coordinate) {
                    let mut perimeter: i64 = 0;
                    let mut area: i64 = 0;

                    measure_plot_perimeter(
                        plot,
                        &coordinate,
                        &plot_data,
                        &mut checked_plots,
                        &mut perimeter,
                        &mut area,
                    );

                    fencing_price += perimeter * area;
                }
            }
        }
        Err(err) => println!("{}", err),
    }

    fencing_price
}

fn get_sides(plot: &HashSet<Coordinate<usize>>) -> i64 {
    let mut sides = 0;

    for square in plot.iter() {
        use Direction::*;

        let unattached = square
            .neighbours()
            .into_iter()
            .filter(|n| !plot.contains(n))
            .count();

        sides += match unattached {
            4 => 4,
            3 => 2,
            2 => {
                if (plot.contains(&square.neighbour(North))
                    && plot.contains(&square.neighbour(South)))
                    || (plot.contains(&square.neighbour(West))
                        || plot.contains(&square.neighbour(East)))
                {
                    0
                } else {
                    1
                }
            }
            _ => 0,
        };

        if !plot.contains(&square.neighbour(NorthEast))
            && plot.contains(&square.neighbour(North))
            && plot.contains(&square.neighbour(East))
        {
            sides += 1;
        }
        if !plot.contains(&square.neighbour(SouthEast))
            && plot.contains(&square.neighbour(South))
            && plot.contains(&square.neighbour(East))
        {
            sides += 1;
        }
        if !plot.contains(&square.neighbour(NorthWest))
            && plot.contains(&square.neighbour(North))
            && plot.contains(&square.neighbour(West))
        {
            sides += 1;
        }
        if !plot.contains(&square.neighbour(SouthWest))
            && plot.contains(&square.neighbour(South))
            && plot.contains(&square.neighbour(West))
        {
            sides += 1;
        }
    }

    sides
}

fn part_two(data: &str) -> i64 {
    let mut result = 0;
    let mut visited: HashSet<Coordinate<usize>> = HashSet::new();

    let mut plot_data: HashMap<Coordinate<usize>, char> = HashMap::new();

    for (y, line) in data.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            plot_data.insert(Coordinate { x, y }, c);
        }
    }

    // match PlotData::new_from_str(data) {
    // Ok(plot_data) => {
    for (coordinate, plot_name) in plot_data.iter() {
        if visited.contains(&coordinate) {
            continue;
        }

        let mut polygon: HashSet<Coordinate<usize>> = HashSet::new();
        polygon.insert(*coordinate);

        let mut not_yet_considered: Vec<Coordinate<usize>> = Vec::new();
        coordinate
            .neighbours()
            .into_iter()
            .filter(|n| !visited.contains(n))
            .for_each(|n| not_yet_considered.push(n));

        while let Some(neighbour) = not_yet_considered.pop() {
            if visited.contains(&neighbour) {
                continue;
            }

            if plot_data.get(&neighbour) == Some(plot_name) {
                visited.insert(neighbour);
                polygon.insert(neighbour);
                neighbour
                    .neighbours()
                    .iter()
                    .filter(|n| !visited.contains(n))
                    .for_each(|n| not_yet_considered.push(*n));
            }
        }

        result += get_sides(&polygon) * polygon.len() as i64;
    }
    //     }
    //     Err(_) => {}
    // }

    result

    // let mut checked_plots: HashSet<Coordinate<usize>> = HashSet::new();
    // let mut fencing_price: i64 = 0;

    // match PlotData::new_from_str(data) {
    //     Ok(plot_data) => {
    //         for (coordinate, &plot_name) in plot_data.enumerate() {
    //             if !checked_plots.contains(&coordinate) {
    //                 let mut plot: HashSet<Coordinate<usize>> = HashSet::new();
    //                 let mut sides = 0;

    //                 measure_plot_sides(
    //                     plot_name,
    //                     &coordinate,
    //                     &plot_data,
    //                     &mut checked_plots,
    //                     &mut plot,
    //                 );

    //                 println!("{:?}", plot);

    //                 sides = get_sides(&plot);

    //                 println!("plot: {}, corners: {}", plot_name, sides);

    //                 fencing_price += sides * (plot.len() as i64);
    //             }
    //         }
    //     }
    //     Err(err) => println!("{}", err),
    // }

    // fencing_price
}

pub fn main() {
    if let Ok(data) = get_challenge_input_as_str(2024, 12) {
        let result = part_one(&data);
        println!("part one: {}", result);
        let result = part_two(&data);
        println!("part two: {}", result);
    }
}

mod tests {
    #![allow(unused)]
    use super::*;

    const EXAMPLE_INPUT: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

    const ABCDE: &str = "AAAA
BBCD
BBCC
EEEC";

    const X_AND_O: &str = "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";

    #[test]
    fn test_parse() {
        let parsed = PlotData::new_from_str(EXAMPLE_INPUT);

        assert!(parsed.is_ok());
        assert_eq!(parsed.as_ref().unwrap().width, 10);
        assert_eq!(parsed.as_ref().unwrap().height, 10);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(ABCDE);
        assert_eq!(result, 140);

        let result = part_one(X_AND_O);
        assert_eq!(result, 772);

        let result = part_one(EXAMPLE_INPUT);
        assert_eq!(result, 1930);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(ABCDE);
        assert_eq!(result, 80);

        let result = part_two(X_AND_O);
        assert_eq!(result, 436);

        let result = part_two(EXAMPLE_INPUT);
        assert_eq!(result, 1206);
    }
}
