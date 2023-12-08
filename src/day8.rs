use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::collections::HashMap;
use std::ops::ControlFlow;

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

    const STARTING_NODE: &str = "AAA";
    const ENDING_NODE: &str = "ZZZ";

    let cf =
        instructions
            .iter()
            .cycle()
            .try_fold(
                (STARTING_NODE, 0),
                |(node, steps), &instruction| match node {
                    ENDING_NODE => ControlFlow::Break(steps),
                    _ => ControlFlow::Continue((
                        map.get(node).unwrap()[instruction as usize].as_str(),
                        steps + 1,
                    )),
                },
            );

    match cf {
        ControlFlow::Continue(_) => panic!("Failed to reach end"),
        ControlFlow::Break(res) => res,
    }
}

#[aoc(day8, part2)]
pub fn solve_part2(input: &(Vec<Instruction>, HashMap<String, [String; 2]>)) -> usize {
    let (instructions, map) = input;

    let starting_nodes = map
        .keys()
        .filter(|node| node.ends_with("A"))
        .map(|node| node.as_str())
        .collect_vec();

    let results = starting_nodes
        .iter()
        .map(|node| {
            let mut current_node = *node;

            instructions
                .iter()
                .cycle()
                .enumerate()
                .find_map(|(index, instruction)| {
                    let options = map
                        .get(current_node)
                        .expect("always exists at a valid node");
                    let next_node = match instruction {
                        Instruction::Left => options[0].as_str(),
                        Instruction::Right => options[1].as_str(),
                    };

                    if next_node.ends_with("Z") {
                        Some(index + 1)
                    } else {
                        current_node = next_node;
                        None
                    }
                })
                .expect("Should find cycle")
        })
        .collect_vec();

    lcm(&results)
}

fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }

    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd(a, b)
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
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

    #[test]
    fn test_vec_lcm() {
        assert_eq!(lcm(&vec![4, 6]), 12);

        assert_eq!(lcm(&vec![4, 6, 5]), 60);
    }
}
