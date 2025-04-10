use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    str::FromStr,
};

use advent_of_code::get_challenge_input_as_str;

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
enum Operation {
    AND,
    OR,
    XOR,
}

#[derive(Hash, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Gate {
    input_a: String,
    operation: Operation,
    input_b: String,
    wire: String,
}

impl FromStr for Gate {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(" ");

        let input_a = parts.next().expect("Error parsing input A").to_string();
        let operation = match parts.next().expect("Invalid operation") {
            "AND" => Ok(Operation::AND),
            "OR" => Ok(Operation::OR),
            "XOR" => Ok(Operation::XOR),
            _ => Err("Invalid parsed value"),
        }
        .expect("Error parsing operation");
        let input_b = parts.next().expect("Error parsing input B").to_string();

        let _ = parts.next();
        let wire = parts.next().expect("Error parsing wire").to_string();

        Ok(Gate {
            input_a,
            operation,
            input_b,
            wire,
        })
    }
}

impl Display for Gate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "input a: {}, operation: {:?}, input b: {} -> {}",
            self.input_a, self.operation, self.input_b, self.wire
        )
    }
}

#[allow(unused)]
fn part_one(data: &str) -> i64 {
    let mut wires: HashMap<String, i64> = HashMap::new();
    let mut gates: HashSet<Gate> = HashSet::new();

    let mut lines = data.lines();
    let mut current_line = lines.next();

    while current_line.is_some_and(|l| !l.is_empty()) {
        let line = current_line.unwrap();
        let wire = &line[0..3];
        let value: i64 = line[5..].parse().unwrap();

        wires.insert(wire.to_string(), value);

        current_line = lines.next();
    }

    current_line = lines.next();

    while current_line.is_some() {
        let gate: Gate = current_line.unwrap().parse().expect("Error parsing gate");
        gates.insert(gate);

        current_line = lines.next();
    }

    while !gates.is_empty() {
        let mut calculated: Vec<Gate> = Vec::new();

        for gate in &gates {
            let input_a = wires.get(&gate.input_a);
            let input_b = wires.get(&gate.input_b);

            if input_a.is_some() && input_b.is_some() {
                use Operation::*;

                wires.insert(
                    gate.wire.clone(),
                    match gate.operation {
                        AND => input_a.unwrap() & input_b.unwrap(),
                        OR => input_a.unwrap() | input_b.unwrap(),
                        XOR => input_a.unwrap() ^ input_b.unwrap(),
                    },
                );

                calculated.push(gate.clone());
            }
        }

        for gate in calculated {
            gates.remove(&gate);
        }
    }

    let mut filtered: Vec<(&String, &i64)> =
        wires.iter().filter(|(k, v)| k.starts_with('z')).collect();
    filtered.sort_by(|(a, _), (b, _)| b.cmp(a));

    let mut binary_string = String::new();
    filtered
        .iter()
        .for_each(|(_, &v)| binary_string.push_str(&v.to_string()));

    i64::from_str_radix(&binary_string, 2).unwrap()
}

#[allow(unused)]
fn part_two(data: &str) -> i32 {
    0
}

pub fn main() {
    if let Ok(data) = get_challenge_input_as_str(2024, 24) {
        let result = part_one(&data);
        println!("part one: {}", result);
        let result = part_two(&data);
        println!("part two: {}", result);
    }
}

mod tests {
    #![allow(unused)]
    use super::*;

    const EXAMPLE_INPUT: &str = "x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(EXAMPLE_INPUT), 2024);
    }

    #[test]
    fn test_part_two() {}
}
