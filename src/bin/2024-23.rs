use std::collections::{HashMap, HashSet};

use advent_of_code::get_challenge_input_as_str;

#[allow(unused)]
fn part_one(data: &str) -> i32 {
    let mut all_connections: HashMap<String, HashSet<String>> = HashMap::new();
    let mut interconnected: i32 = 0;

    for line in data.lines() {
        let computer_a = &line[0..2];
        let computer_b = &line[3..5];

        all_connections
            .entry(computer_a.to_string())
            .or_insert_with(HashSet::new)
            .insert(computer_b.to_string());

        all_connections
            .entry(computer_b.to_string())
            .or_insert_with(HashSet::new)
            .insert(computer_a.to_string());
    }

    let mut triples: Vec<HashSet<String>> = Vec::new();

    for (computer, connected_computers) in &all_connections {
        let connected_computers: Vec<_> = connected_computers.iter().collect();
        for i in 0..connected_computers.len() {
            for j in i + 1..connected_computers.len() {
                let computer_a = connected_computers[i];
                let computer_b = connected_computers[j];

                if all_connections
                    .get(computer_a)
                    .map_or(false, |n| n.contains(computer_b))
                {
                    let triple: HashSet<String> =
                        HashSet::from([computer.clone(), computer_a.clone(), computer_b.clone()]);

                    if !triples.contains(&triple) {
                        triples.push(triple);
                    }
                }
            }
        }
    }

    triples
        .iter()
        .filter(|t| t.iter().any(|c| c.starts_with("t")))
        .count() as i32
}

#[allow(unused)]
fn part_two(data: &str) -> i32 {
    0
}

pub fn main() {
    if let Ok(data) = get_challenge_input_as_str(2024, 23) {
        let result = part_one(&data);
        println!("part one: {}", result);
        let result = part_two(&data);
        println!("part two: {}", result);
    }
}

mod tests {
    #![allow(unused)]
    use super::*;

    const EXAMPLE_INPUT: &str = "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(EXAMPLE_INPUT), 7);
    }

    #[test]
    fn test_part_two() {}
}
