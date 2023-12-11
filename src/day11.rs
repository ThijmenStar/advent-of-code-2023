use aoc_runner_derive::{aoc, aoc_generator};
use glam::IVec2;
use itertools::Itertools;

const EXPANSION_FACTOR: i32 = 1_000_000;

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
pub fn solve_part1(universe: &[IVec2]) -> u64 {
    solve_for_factor(universe, 2)
}


#[aoc(day11, part2)]
pub fn solve_part2(universe: &[IVec2]) -> u64 {
    solve_for_factor(universe, EXPANSION_FACTOR)
}

fn solve_for_factor(universe: &[IVec2], factor: i32) -> u64 {
    let exp_universe = expand_universe(universe, factor);

    exp_universe
        .iter()
        .tuple_combinations()
        .map(|(a, b)| manhattan_distance(a, b) as u64)
        .sum()
}

fn expand_universe(universe: &[IVec2], factor: i32) -> Vec<IVec2> {
    let mut offset = IVec2::new(0, 0);
    let mut exp_universe: Vec<IVec2> = [IVec2::new(0, 0)]
        .iter()
        .chain(universe.iter())
        .tuple_windows()
        .map(|(a, b)| {
            if a.y != b.y {
                let dots = b.y - a.y - 1;
                offset.y += dots * factor - dots;
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
                let dots = b.x - a.x - 1;
                offset.x += dots * factor - dots;
            }
            b + offset
        }).collect()
    ;
    exp_universe
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

    const SIMPLE_INPUT: &str = "#..#
....
#..#";

    #[test]
    fn parse_example() {
        assert_eq!(
            parse_input(EXAMPLE_INPUT),
            vec![
                IVec2::new(3, 0),
                IVec2::new(7, 1),
                IVec2::new(0, 2),
                IVec2::new(6, 4),
                IVec2::new(1, 5),
                IVec2::new(9, 6),
                IVec2::new(7, 8),
                IVec2::new(0, 9),
                IVec2::new(4, 9),
            ]
        )
    }

    #[test]
    fn solve_example_part1() {
        assert_eq!(solve_part1(&parse_input(EXAMPLE_INPUT)), 374)
    }

    #[test]
    fn test_expansion() {
        assert_eq!(expand_universe(&parse_input(SIMPLE_INPUT), 10),
            vec![IVec2::new(0, 0), IVec2::new(0, 11), IVec2::new(21, 0), IVec2::new(21, 11)]
        )
    }

    #[test]
    fn solve_example_factor_10() {
        assert_eq!(solve_for_factor(&parse_input(EXAMPLE_INPUT), 10), 1030)
    }

    #[test]
    fn solve_example_factor_100() {
        assert_eq!(solve_for_factor(&parse_input(EXAMPLE_INPUT), 100), 8410)
    }


    #[test]
    fn solve_simple_factor_10() {
        assert_eq!(solve_for_factor(&parse_input(SIMPLE_INPUT), 10), 11*2 + 21*2 + 32*2)
    }
}
