use aoc_runner_derive::{aoc, aoc_generator};
use glam::IVec2;
use itertools::Itertools;
use nom::branch::alt;
use nom::bytes::complete::{tag, take_while_m_n};
use nom::character::complete;
use nom::character::complete::{line_ending, space1};
use nom::combinator::map_res;
use nom::multi::separated_list1;
use nom::sequence::{delimited, tuple};
use nom::IResult;
use nom::Parser;
use std::collections::{HashSet, VecDeque};

#[derive(Debug, PartialEq)]
struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

#[derive(Debug, PartialEq)]
struct Instruction {
    direction: IVec2,
    length: i32,
    color: Color,
}

fn from_hex(input: &str) -> Result<u8, std::num::ParseIntError> {
    u8::from_str_radix(input, 16)
}

fn is_hex_digit(c: char) -> bool {
    c.is_digit(16)
}

fn hex_primary(input: &str) -> IResult<&str, u8> {
    map_res(take_while_m_n(2, 2, is_hex_digit), from_hex)(input)
}

fn hex_color(input: &str) -> IResult<&str, Color> {
    let (input, _) = tag("#")(input)?;
    let (input, (red, green, blue)) = tuple((hex_primary, hex_primary, hex_primary))(input)?;

    Ok((input, Color { red, green, blue }))
}

fn instruction(input: &str) -> IResult<&str, Instruction> {
    let (input, direction) = alt((
        complete::char('U').map(|_| IVec2::NEG_Y),
        complete::char('D').map(|_| IVec2::Y),
        complete::char('L').map(|_| IVec2::NEG_X),
        complete::char('R').map(|_| IVec2::X),
    ))(input)?;
    let (input, length) = delimited(space1, complete::i32, space1)(input)?;

    let (input, color) = delimited(complete::char('('), hex_color, complete::char(')'))(input)?;

    Ok((
        input,
        Instruction {
            direction,
            length,
            color,
        },
    ))
}

fn instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
    separated_list1(line_ending, instruction)(input)
}

#[aoc_generator(day18)]
fn parse_input(input: &str) -> Vec<Instruction> {
    instructions(input).unwrap().1
}

#[aoc(day18, part1)]
fn solve_part1(input: &Vec<Instruction>) -> i32 {
    let mut min_pos = IVec2::MAX;
    let mut max_pos = IVec2::MIN;

    // Dig trenches
    let (_, trenches) = input.iter().fold(
        (IVec2::ZERO, HashSet::from([IVec2::ZERO])),
        |(mut position, mut dug_out), instruction| {
            for _ in 0..instruction.length {
                position += instruction.direction;
                dug_out.insert(position);
                min_pos = min_pos.min(position);
                max_pos = max_pos.max(position);
            }
            (position, dug_out)
        },
    );

    let mut outside: HashSet<IVec2> = HashSet::new();
    let mut queue: VecDeque<IVec2> = VecDeque::new();

    // Add outside edges
    outside.extend(
        (min_pos.x..max_pos.x + 1)
            .map(|x| IVec2::new(x, min_pos.y))
            .filter(|pos| !trenches.contains(pos)),
    );
    outside.extend(
        (min_pos.x..max_pos.x + 1)
            .map(|x| IVec2::new(x, max_pos.y))
            .filter(|pos| !trenches.contains(pos)),
    );
    outside.extend(
        (min_pos.y..max_pos.y + 1)
            .map(|y| IVec2::new(min_pos.x, y))
            .filter(|pos| !trenches.contains(pos)),
    );
    outside.extend(
        (min_pos.y..max_pos.y + 1)
            .map(|y| IVec2::new(max_pos.x, y))
            .filter(|pos| !trenches.contains(pos)),
    );

    queue.extend(outside.clone());

    while let Some(pos) = queue.pop_front() {
        const DIRECTIONS: [IVec2; 4] = [IVec2::NEG_X, IVec2::X, IVec2::NEG_Y, IVec2::Y];

        let new_pos = DIRECTIONS
            .into_iter()
            .map(|dir| pos + dir)
            .filter(|pos| pos.cmpge(min_pos).all() && pos.cmple(max_pos).all()) // inside min max
            .filter(|pos| !trenches.contains(pos) && !outside.contains(pos))
            .collect_vec();

        queue.extend(new_pos.clone());
        outside.extend(new_pos);
    }

    let size = max_pos - min_pos;
    dbg!(size);

    (size.x + 1) * (size.y + 1) - outside.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";

    #[test]
    fn parse_color() {
        assert_eq!(
            hex_color("#2F14DF"),
            Ok((
                "",
                Color {
                    red: 47,
                    green: 20,
                    blue: 223,
                }
            ))
        );
    }

    #[test]
    fn parse_example_input() {
        let res = parse_input(EXAMPLE_INPUT);

        assert_eq!(res.len(), 14);
        assert_eq!(
            res.first(),
            Some(&Instruction {
                direction: IVec2::X,
                length: 6,
                color: Color {
                    red: 112,
                    green: 199,
                    blue: 16,
                },
            })
        )
    }

    #[test]
    fn solve_example_part1() {
        assert_eq!(solve_part1(&parse_input(EXAMPLE_INPUT)), 62);
    }
}
