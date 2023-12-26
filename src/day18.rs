use aoc_runner_derive::{aoc, aoc_generator};
use glam::I64Vec2;
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

#[derive(Debug, PartialEq)]
struct Instruction {
    direction: I64Vec2,
    distance: i64,
}

fn from_hex(input: &str) -> Result<u32, std::num::ParseIntError> {
    u32::from_str_radix(input, 16)
}

fn is_hex_digit(c: char) -> bool {
    c.is_digit(16)
}

fn hex_distance(input: &str) -> IResult<&str, u32> {
    map_res(take_while_m_n(5, 5, is_hex_digit), from_hex)(input)
}

fn hex_direction(input: &str) -> IResult<&str, I64Vec2> {
    Ok(alt((
        complete::char('0').map(|_| I64Vec2::X),
        complete::char('1').map(|_| I64Vec2::Y),
        complete::char('2').map(|_| I64Vec2::NEG_X),
        complete::char('3').map(|_| I64Vec2::NEG_Y),
    ))(input)?)
}

fn hex_instruction(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("#")(input)?;
    let (input, (distance, direction)) = tuple((hex_distance, hex_direction))(input)?;

    Ok((
        input,
        Instruction {
            direction,
            distance: distance as i64,
        },
    ))
}

fn instruction(input: &str) -> IResult<&str, (Instruction, Instruction)> {
    let (input, direction) = alt((
        complete::char('U').map(|_| I64Vec2::NEG_Y),
        complete::char('D').map(|_| I64Vec2::Y),
        complete::char('L').map(|_| I64Vec2::NEG_X),
        complete::char('R').map(|_| I64Vec2::X),
    ))(input)?;
    let (input, distance) = delimited(space1, complete::i64, space1)(input)?;

    let (input, instruction2) =
        delimited(complete::char('('), hex_instruction, complete::char(')'))(input)?;

    Ok((
        input,
        (
            Instruction {
                direction,
                distance,
            },
            instruction2,
        ),
    ))
}

fn instructions(input: &str) -> IResult<&str, Vec<(Instruction, Instruction)>> {
    separated_list1(line_ending, instruction)(input)
}

#[aoc_generator(day18, part1)]
fn parse_input_part1(input: &str) -> Vec<Instruction> {
    instructions(input)
        .unwrap()
        .1
        .into_iter()
        .map(|ins| ins.0)
        .collect()
}

#[aoc_generator(day18, part2)]
fn parse_input_part2(input: &str) -> Vec<Instruction> {
    instructions(input)
        .unwrap()
        .1
        .into_iter()
        .map(|ins| ins.1)
        .collect()
}

#[aoc(day18, part1)]
fn solve_part1(input: &Vec<Instruction>) -> i64 {
    solve(input)
}

#[aoc(day18, part2)]
fn solve_part2(input: &Vec<Instruction>) -> i64 {
    solve(input)
}

fn solve(input: &Vec<Instruction>) -> i64 {
    let (_, trench_path, total_distance) = input.iter().fold(
        (I64Vec2::ZERO, vec![I64Vec2::ZERO], 0),
        |(position, mut path, distance), instruction| {
            let new_pos = position + instruction.direction * instruction.distance;
            path.push(new_pos);
            (new_pos, path, distance + instruction.distance)
        },
    );

    let area_polygon = trench_path
        .iter()
        .tuple_windows()
        .map(|(a, b)| (a.x * b.y) - (b.x * a.y))
        .sum::<i64>()
        / 2; // shoelace formula

    area_polygon + total_distance / 2 + 1 // picks theorem
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
    fn parse_hex() {
        assert_eq!(
            hex_instruction("#70c710"),
            Ok((
                "",
                Instruction {
                    direction: I64Vec2::X,
                    distance: 461937,
                }
            ))
        );
    }

    #[test]
    fn parse_example_input_part1() {
        let res = parse_input_part1(EXAMPLE_INPUT);

        assert_eq!(res.len(), 14);
        assert_eq!(
            res.first(),
            Some(&Instruction {
                direction: I64Vec2::X,
                distance: 6,
            })
        )
    }

    #[test]
    fn solve_example_part1() {
        assert_eq!(solve_part1(&parse_input_part1(EXAMPLE_INPUT)), 62);
    }

    #[test]
    fn solve_example_part2() {
        assert_eq!(solve_part2(&parse_input_part2(EXAMPLE_INPUT)), 952408144115);
    }
}
