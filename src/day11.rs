use aoc_runner_derive::{aoc, aoc_generator};
use glam::IVec2;
use itertools::Itertools;

#[aoc_generator(day11)]
pub fn parse_input(input: &str) -> Vec<IVec2> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().flat_map(move |(x, c)| match c {
                '#' => Some(IVec2::new(x as i32, y as i32)),
                _ => None,
            })
        })
        .collect()
}

#[aoc(day11, part1)]
pub fn solve_part1(universe: &[IVec2]) -> u32 {
    // expand universe
    let mut offset = IVec2::new(0, 0);
    let mut exp_universe: Vec<IVec2> = [IVec2::new(0, 0)]
        .iter()
        .chain(universe.iter())
        .tuple_windows()
        .map(|(a, b)| {
            if a.y != b.y {
                offset.y += b.y - a.y - 1;
            }
            *b + offset
        })
        .collect();

    offset = IVec2::new(0, 0);
    exp_universe = [IVec2::new(0, 0)]
        .into_iter()
        .chain(exp_universe)
        .into_iter()
        .sorted_by_key(|pos| (pos.x, pos.y))
        .tuple_windows()
        .map(|(a, b)| {
            if a.x != b.x {
                offset.x += b.x - a.x - 1;
            }
            b + offset
        }).collect()
    ;

    exp_universe
        .iter()
        .tuple_combinations()
        .map(|(a, b)| manhattan_distance(a, b))
        .sum()
}

fn manhattan_distance(a: &IVec2, b: &IVec2) -> u32 {
    a.x.abs_diff(b.x) + a.y.abs_diff(b.y)
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_INPUT: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    #[test]
    fn parse_example() {
        assert_eq!(
            parse_input(EXAMPLE_INPUT),
            vec![
                IVec2::new(4, 0),
                IVec2::new(9, 1),
                IVec2::new(0, 2),
                IVec2::new(8, 5),
                IVec2::new(1, 6),
                IVec2::new(12, 7),
                IVec2::new(9, 10),
                IVec2::new(0, 11),
                IVec2::new(5, 11),
            ]
        )
    }

    #[test]
    fn solve_example_part1() {
        assert_eq!(solve_part1(&parse_input(EXAMPLE_INPUT)), 374)
    }
}
