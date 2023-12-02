use std::cmp;
use aoc_runner_derive::{aoc, aoc_generator};

#[derive(PartialEq, Debug)]
pub struct Game {
    id: u32,
    subsets: Vec<Vec<CubeCount>>,
}

#[derive(PartialEq, Debug)]
enum Color {
    RED,
    GREEN,
    BLUE,
}

#[derive(PartialEq, Debug)]
pub struct CubeCount(Color, u32);

#[aoc_generator(day2)]
pub fn parse_input(input: &str) -> Vec<Game> {
    input
        .lines()
        .map(|l| {
            let (metadata, data) = l.split_once(':').expect("Failed to split on ':'");
            let id = metadata
                .split_whitespace()
                .last()
                .expect("Invalid metadata")
                .parse()
                .expect("Invalid id");
            let subsets = data
                .split(";")
                .map(|s| {
                    {
                        s.split(',').map(|c| match c.trim().split_once(' ') {
                            Some((n, "red")) => {
                                CubeCount(Color::RED, n.parse().expect("Invalid cube count"))
                            }
                            Some((n, "green")) => {
                                CubeCount(Color::GREEN, n.parse().expect("Invalid cube count"))
                            }
                            Some((n, "blue")) => {
                                CubeCount(Color::BLUE, n.parse().expect("Invalid cube count"))
                            }
                            a => {
                                panic!("Unknown cubes: {:?}", a)
                            }
                        })
                    }
                    .collect()
                })
                .collect();

            Game { id, subsets }
        })
        .collect()
}

const RED_CUBES: u32 = 12;
const GREEN_CUBES: u32 = 13;
const BLUE_CUBES: u32 = 14;

#[aoc(day2, part1)]
pub fn solve_part1(input: &[Game]) -> u32 {
    input
        .iter()
        .filter(|game| {
            game.subsets.iter().flatten().all(|cubes| match cubes {
                CubeCount(Color::RED, n) => *n <= RED_CUBES,
                CubeCount(Color::GREEN, n) => *n <= GREEN_CUBES,
                CubeCount(Color::BLUE, n) => *n <= BLUE_CUBES,
            })
        })
        .map(|g| g.id)
        .sum()
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &[Game]) -> u32 {
    input.iter()
        .map(|game| {
            game.subsets.iter().flatten().fold([0, 0, 0], |mut max, cube_count| {
                match cube_count {
                    CubeCount(Color::RED, n) => max[0] = cmp::max(max[0], *n),
                    CubeCount(Color::GREEN, n) => max[1] = cmp::max(max[1], *n),
                    CubeCount(Color::BLUE, n) => max[2] = cmp::max(max[2], *n),
                };
                max
            }).iter().product::<u32>()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn parse_example() {
        assert_eq!(
            parse_input("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),
            vec![Game {
                id: 1,
                subsets: vec![
                    vec![CubeCount(Color::BLUE, 3), CubeCount(Color::RED, 4)],
                    vec![
                        CubeCount(Color::RED, 1),
                        CubeCount(Color::GREEN, 2),
                        CubeCount(Color::BLUE, 6)
                    ],
                    vec![CubeCount(Color::GREEN, 2)],
                ]
            }]
        )
    }

    #[test]
    fn solve_part1_example() {
        let input = parse_input(EXAMPLE_INPUT);

        assert_eq!(solve_part1(&input), 8)
    }

    #[test]
    fn solve_part2_example() {
        let input = parse_input(EXAMPLE_INPUT);

        assert_eq!(solve_part2(&input), 2286)
    }
}
