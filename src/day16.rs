use crate::day16::Direction::{Down, Left, Right, Up};
use aoc_runner_derive::{aoc, aoc_generator};
use glam::IVec2;
use itertools::Itertools;
use std::collections::HashSet;

#[derive(Eq, PartialEq)]
enum MirrorType {
    HorizontalSplit, // |
    VerticalSplit,   // -
    Upward,          // /
    Downward,        // \
}

struct Mirror {
    pos: IVec2,
    mirror_type: MirrorType,
}

struct Contraption {
    mirrors: Vec<Mirror>,
    rows: usize,
    cols: usize,
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn invert(self) -> Direction {
        match self {
            Up => Down,
            Down => Up,
            Left => Right,
            Right => Left,
        }
    }
}

#[aoc_generator(day16)]
fn parse_input(input: &str) -> Contraption {
    let mirrors = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().flat_map(move |(x, c)| match c {
                '.' => None,
                '|' => Some(Mirror {
                    pos: IVec2::new(x as i32, y as i32),
                    mirror_type: MirrorType::HorizontalSplit,
                }),
                '-' => Some(Mirror {
                    pos: IVec2::new(x as i32, y as i32),
                    mirror_type: MirrorType::VerticalSplit,
                }),
                '/' => Some(Mirror {
                    pos: IVec2::new(x as i32, y as i32),
                    mirror_type: MirrorType::Upward,
                }),
                '\\' => Some(Mirror {
                    pos: IVec2::new(x as i32, y as i32),
                    mirror_type: MirrorType::Downward,
                }),
                _ => panic!("Unknown symbol"),
            })
        })
        .collect();

    let rows = input.lines().count();
    let cols = input.lines().next().unwrap().len();

    Contraption {
        mirrors,
        rows,
        cols,
    }
}

#[aoc(day16, part1)]
fn solve_part1(input: &Contraption) -> usize {
    shoot_and_count(input, Right, 0)
}

#[aoc(day16, part2)]
fn solve_part2(input: &Contraption) -> usize {
    [Up, Down, Left, Right]
        .into_iter()
        .flat_map(|initial_direction| {
            assert_eq!(input.rows, input.cols);
            (0..input.cols)
                .map(|offset| shoot_and_count(input, initial_direction, offset))
                .max()
        })
        .max()
        .unwrap()
}

fn shoot_and_count(input: &Contraption, initial_direction: Direction, offset: usize) -> usize {
    let mut energized: HashSet<IVec2> = HashSet::new();
    let mut reflections: HashSet<(IVec2, Direction)> = HashSet::new();

    let first_mirror = match initial_direction {
        Up | Down => input
            .mirrors
            .iter()
            .find_position(|m| m.pos.x == offset as i32),
        Left | Right => input
            .mirrors
            .iter()
            .find_position(|m| m.pos.y == offset as i32),
    };

    let initial_position = match initial_direction {
        Up => IVec2::new(offset as i32, input.rows as i32 - 1),
        Down => IVec2::new(offset as i32, 0),
        Left => IVec2::new(input.cols as i32 - 1, offset as i32),
        Right => IVec2::new(0, offset as i32),
    };

    match first_mirror {
        Some((index, mirror)) => {
            energized.extend(ivec2_range(&initial_position, &mirror.pos));

            reflect_in(
                input,
                &mut reflections,
                &mut energized,
                index,
                initial_direction,
            );

            energized.len()
        }
        None => input.rows, // beam does not hit any mirrors
    }
}
fn reflect_in(
    contraption: &Contraption,
    reflections: &mut HashSet<(IVec2, Direction)>,
    energized: &mut HashSet<IVec2>,
    mirror_index: usize,
    beam_in: Direction,
) {
    let mirror = &contraption.mirrors[mirror_index];

    // if a beam is coming in from a direction we no longer have to explore in that direction
    reflections.insert((mirror.pos, beam_in.invert()));

    match mirror.mirror_type {
        MirrorType::HorizontalSplit => match beam_in {
            Left | Right => {
                reflect_out(contraption, reflections, energized, mirror_index, Up);
                reflect_out(contraption, reflections, energized, mirror_index, Down);
            }
            Up | Down => reflect_out(contraption, reflections, energized, mirror_index, beam_in),
        },
        MirrorType::VerticalSplit => match beam_in {
            Up | Down => {
                reflect_out(contraption, reflections, energized, mirror_index, Left);
                reflect_out(contraption, reflections, energized, mirror_index, Right);
            }
            Left | Right => reflect_out(contraption, reflections, energized, mirror_index, beam_in),
        },
        MirrorType::Upward => match beam_in {
            Up => reflect_out(contraption, reflections, energized, mirror_index, Right),
            Down => reflect_out(contraption, reflections, energized, mirror_index, Left),
            Left => reflect_out(contraption, reflections, energized, mirror_index, Down),
            Right => reflect_out(contraption, reflections, energized, mirror_index, Up),
        },
        MirrorType::Downward => match beam_in {
            Up => reflect_out(contraption, reflections, energized, mirror_index, Left),
            Down => reflect_out(contraption, reflections, energized, mirror_index, Right),
            Left => reflect_out(contraption, reflections, energized, mirror_index, Up),
            Right => reflect_out(contraption, reflections, energized, mirror_index, Down),
        },
    }
}

fn reflect_out(
    contraption: &Contraption,
    reflections: &mut HashSet<(IVec2, Direction)>,
    energized: &mut HashSet<IVec2>,
    mirror_index: usize,
    beam_out: Direction,
) {
    let current_mirror = &contraption.mirrors[mirror_index];

    // check if reflection is already followed
    if !reflections.insert((current_mirror.pos, beam_out)) {
        return;
    }

    let next_mirror_index = match beam_out {
        Up => contraption.mirrors[..mirror_index]
            .iter()
            .rev()
            .position(|m| m.pos.x == current_mirror.pos.x)
            .map(|offset| mirror_index.checked_sub(offset + 1).unwrap()),
        Down => contraption.mirrors[mirror_index + 1..]
            .iter()
            .position(|m| m.pos.x == current_mirror.pos.x)
            .map(|offset| mirror_index.checked_add(offset + 1).unwrap()),
        Left => contraption.mirrors[..mirror_index]
            .iter()
            .rev()
            .position(|m| m.pos.y == current_mirror.pos.y)
            .map(|offset| mirror_index.checked_sub(offset + 1).unwrap()),
        Right => contraption.mirrors[mirror_index + 1..]
            .iter()
            .position(|m| m.pos.y == current_mirror.pos.y)
            .map(|offset| mirror_index.checked_add(offset + 1).unwrap()),
    };

    match next_mirror_index {
        Some(i) => {
            energized.extend(ivec2_range(
                &current_mirror.pos,
                &contraption.mirrors[i].pos,
            ));

            reflect_in(contraption, reflections, energized, i, beam_out);
        }
        None => energized.extend(match beam_out {
            // beam goes of screen
            Up => ivec2_range(&IVec2::new(current_mirror.pos.x, 0), &(current_mirror.pos)),
            Down => ivec2_range(
                &current_mirror.pos,
                &IVec2::new(current_mirror.pos.x, contraption.rows as i32 - 1),
            ),
            Left => ivec2_range(&IVec2::new(0, current_mirror.pos.y), &(current_mirror.pos)),
            Right => ivec2_range(
                &current_mirror.pos,
                &IVec2::new(contraption.cols as i32 - 1, current_mirror.pos.y),
            ),
        }),
    }
}

fn ivec2_range(start: &IVec2, end: &IVec2) -> Vec<IVec2> {
    if start.x == end.x {
        (start.y.min(end.y)..start.y.max(end.y) + 1)
            .map(|y| IVec2::new(start.x, y))
            .collect()
    } else if start.y == end.y {
        (start.x.min(end.x)..start.x.max(end.x) + 1)
            .map(|x| IVec2::new(x, start.y))
            .collect()
    } else {
        dbg!(start, end);
        panic!("Only horizontal and vertical ranges supported")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = ".|...\\....
|.-.\\.....
.....|-...
........|.
..........
.........\\
..../.\\\\..
.-.-/..|..
.|....-|.\\
..//.|....";

    const SIMPLE_INPUT: &str = ".|..\\
...//
.-./.";

    #[test]
    fn solve_example_part1() {
        assert_eq!(solve_part1(&parse_input(EXAMPLE_INPUT)), 46);
    }
    #[test]
    fn solve_simple_part1() {
        assert_eq!(solve_part1(&parse_input(SIMPLE_INPUT)), 12);
    }

    #[test]
    fn solve_example_part2() {
        assert_eq!(solve_part2(&parse_input(EXAMPLE_INPUT)), 51);
    }

    #[test]
    fn test_ivec2_range() {
        assert_eq!(
            ivec2_range(&IVec2::ZERO, &IVec2::new(4, 0)),
            vec![
                IVec2::new(0, 0),
                IVec2::new(1, 0),
                IVec2::new(2, 0),
                IVec2::new(3, 0),
                IVec2::new(4, 0),
            ]
        )
    }
}
