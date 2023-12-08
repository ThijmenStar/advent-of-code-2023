use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;
use std::ops::ControlFlow;
use itertools::Itertools;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Instruction {
    Left = 0,
    Right = 1,
}

#[aoc_generator(day8)]
pub fn parse_input(input: &str) -> (Vec<Instruction>, HashMap<String, [String; 2]>) {
    let (instructions, map) = input
        .split_once("\n\n")
        .expect("Failed to split instructions and map");

    (
        instructions
            .chars()
            .map(|i| match i {
                'L' => Instruction::Left,
                'R' => Instruction::Right,
                _ => panic!("Invalid instruction"),
            })
            .collect(),
        HashMap::from_iter(map.lines().map(|line| {
            (
                line[0..3].to_string(),
                [line[7..10].to_string(), line[12..15].to_string()],
            )
        })),
    )
}

#[aoc(day8, part1)]
pub fn solve_part1(input: &(Vec<Instruction>, HashMap<String, [String; 2]>)) -> u32 {
    let (instructions, map) = input;

    let starting_node = "AAA";

    let cf = instructions
        .iter()
        .cycle()
        .try_fold(
            (starting_node, 0),
            |(node, steps), &instruction| match node {
                "ZZZ" => ControlFlow::Break(steps),
                _ => ControlFlow::Continue((map.get(node).unwrap()[instruction as usize].as_str(), steps + 1)),
            },
        );

    match cf {
        ControlFlow::Continue(_) => panic!("Failed to reach end"),
        ControlFlow::Break(res) => res,
    }
}

#[aoc(day8, part2)]
pub fn solve_part2(input: &(Vec<Instruction>, HashMap<String, [String; 2]>)) -> u32 {
    let (instructions, map) = input;

    let starting_nodes = map.keys().filter(|node| node.ends_with("A")).map(|node| node.as_str()).collect_vec();


    let cf = instructions
        .iter()
        .cycle()
        .try_fold(
            (starting_nodes, 0),
            |(nodes, steps), &instruction| {
                match nodes.iter().filter(|node| node.ends_with("Z")).count() > 2 {
                    true => ControlFlow::Break(steps),
                    false => ControlFlow::Continue((
                        nodes.into_iter().map(|node| map.get(node).unwrap()[instruction as usize].as_str()).collect(),
                        steps + 1
                    ))
                }
            },
        );

    match cf {
        ControlFlow::Continue(_) => panic!("Failed to reach end"),
        ControlFlow::Break(res) => res,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    const EXAMPLE_INPUT_2: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

    #[test]
    fn example_input() {
        assert_eq!(
            parse_input(EXAMPLE_INPUT),
            (
                vec![Instruction::Left, Instruction::Left, Instruction::Right],
                HashMap::from([
                    ("AAA".to_string(), ["BBB".to_string(), "BBB".to_string()]),
                    ("BBB".to_string(), ["AAA".to_string(), "ZZZ".to_string()]),
                    ("ZZZ".to_string(), ["ZZZ".to_string(), "ZZZ".to_string()]),
                ])
            )
        )
    }


    #[test]
    fn solve_example_part1() {
        assert_eq!(solve_part1(&parse_input(EXAMPLE_INPUT)), 6)
    }

    #[test]
    fn solve_example_part2() {
        assert_eq!(solve_part2(&parse_input(EXAMPLE_INPUT_2)), 6)
    }
}
