use aoc_runner_derive::{aoc, aoc_generator};
use glam::IVec2;
use itertools::Itertools;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::hash::Hash;

const CYCLES: usize = 1_000_000_000;

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
enum RockType {
    Cube,
    Round,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
struct Tile {
    loc: IVec2,
    rock_type: RockType,
}

#[derive(Clone, Eq, PartialEq)]
struct Input {
    rocks: Vec<Tile>,
    rows: usize,
    cols: usize,
}

impl Debug for Input {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut tiles = vec![vec!['.'; self.cols]; self.rows];

        self.rocks.iter().for_each(|t| {
            tiles[t.loc.y as usize][t.loc.x as usize] = match t.rock_type {
                RockType::Cube => '#',
                RockType::Round => 'O',
            }
        });

        let output: String = tiles
            .iter()
            .map(|line| line.iter().collect::<String>())
            .join("\n");

        write!(f, "{}", output)
    }
}

#[aoc_generator(day14)]
fn parse_input(input: &str) -> Input {
    Input {
        rocks: input
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars().enumerate().flat_map(move |(x, c)| match c {
                    '#' => Some(Tile {
                        loc: IVec2::new(x as i32, y as i32),
                        rock_type: RockType::Cube,
                    }),
                    'O' => Some(Tile {
                        loc: IVec2::new(x as i32, y as i32),
                        rock_type: RockType::Round,
                    }),
                    '.' => None,
                    _ => panic!("Invalid input"),
                })
            })
            .collect(),
        rows: input.lines().count(),
        cols: input.lines().next().unwrap().len(),
    }
}

#[aoc(day14, part1)]
fn solve_part1(input: &Input) -> usize {
    let mut rocks = Vec::from(input.rocks.as_slice());
    slide_rocks_north(&mut rocks);

    rocks
        .iter()
        .filter(|t| t.rock_type == RockType::Round)
        .map(|t| input.rows - t.loc.y as usize)
        .sum()
}

#[aoc(day14, part2)]
fn solve_part2(input: &Input) -> usize {
    let mut rocks = Vec::from(input.rocks.as_slice());
    slide_rocks_north(&mut rocks);

    let mut cycles_map: HashMap<Vec<Tile>, usize> = HashMap::new();
    cycles_map.insert(rocks.clone(), 0);

    let (loop_start, loop_end) = (1..CYCLES + 1)
        .find_map(|n| {
            cycle(&mut rocks, input.rows, input.cols);
            match cycles_map.get(&rocks) {
                None => {
                    cycles_map.insert(rocks.clone(), n);
                    None
                }
                Some(&j) => Some((j, n)),
            }
        })
        .expect("no loop found"); // will probably run out of memory before you reach this, found out the hard way haha

    let left_over_cycles = (CYCLES - loop_start) % (loop_end - loop_start);

    (0..left_over_cycles).for_each(|_| {
        cycle(&mut rocks, input.rows, input.cols);
    });

    rocks
        .iter()
        .filter(|t| t.rock_type == RockType::Round)
        .map(|t| input.rows - t.loc.y as usize)
        .sum()
}

fn cycle(rocks: &mut Vec<Tile>, rows: usize, cols: usize) {
    slide_rocks_north(rocks);
    slide_rocks_west(rocks);
    slide_rocks_south(rocks, rows);
    slide_rocks_east(rocks, cols)
}

fn slide_rocks_north(rocks: &mut Vec<Tile>) {
    rocks.sort_by_key(|t| t.loc.y);

    let round_positions: Vec<usize> = rocks
        .iter()
        .positions(|t| t.rock_type == RockType::Round)
        .collect();

    for i in round_positions {
        let x = rocks[i].loc.x;
        rocks[i].loc.y = rocks[..i]
            .iter()
            .rfind(|t| t.loc.x == x)
            .map(|t| t.loc.y + 1)
            .unwrap_or(0);
    }
}

fn slide_rocks_west(rocks: &mut Vec<Tile>) {
    rocks.sort_by_key(|t| t.loc.x);

    let round_positions: Vec<usize> = rocks
        .iter()
        .positions(|t| t.rock_type == RockType::Round)
        .collect();

    for i in round_positions {
        let y = rocks[i].loc.y;
        rocks[i].loc.x = rocks[..i]
            .iter()
            .rfind(|t| t.loc.y == y)
            .map(|t| t.loc.x + 1)
            .unwrap_or(0);
    }
}

fn slide_rocks_south(rocks: &mut Vec<Tile>, rows: usize) {
    rocks.sort_by_key(|t| t.loc.y);

    let round_positions: Vec<usize> = rocks
        .iter()
        .positions(|t| t.rock_type == RockType::Round)
        .rev()
        .collect();

    for i in round_positions {
        let x = rocks[i].loc.x;
        rocks[i].loc.y = rocks[i + 1..]
            .iter()
            .find(|t| t.loc.x == x)
            .map(|t| t.loc.y - 1)
            .unwrap_or(rows as i32 - 1);
    }
}

fn slide_rocks_east(rocks: &mut Vec<Tile>, cols: usize) {
    rocks.sort_by_key(|t| t.loc.x);

    let round_positions: Vec<usize> = rocks
        .iter()
        .positions(|t| t.rock_type == RockType::Round)
        .rev()
        .collect();

    for i in round_positions {
        let y = rocks[i].loc.y;
        rocks[i].loc.x = rocks[i + 1..]
            .iter()
            .find(|t| t.loc.y == y)
            .map(|t| t.loc.x - 1)
            .unwrap_or(cols as i32 - 1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

    const EXAMPLE_RESULT: &str = "OOOO.#.O..
OO..#....#
OO..O##..O
O..#.OO...
........#.
..#....#.#
..O..#.O.O
..O.......
#....###..
#....#....";

    const EXAMPLE_RESULT_SLIDE_EAST: &str = "....O#....
.OOO#....#
.....##...
.OO#....OO
......OO#.
.O#...O#.#
....O#..OO
.........O
#....###..
#..OO#....";

    const AFTER_1_CYCLE: &str = ".....#....
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#....";

    const AFTER_3_CYCLES: &str = ".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#...O###.O
#.OOO#...O";

    #[test]
    fn test_slide_rocks() {
        let mut input = parse_input(EXAMPLE_INPUT);
        slide_rocks_north(&mut input.rocks);
        input.rocks.sort_by_key(|t| (t.loc.y, t.loc.x));

        assert_eq!(input.rocks, parse_input(EXAMPLE_RESULT).rocks);
    }

    #[test]
    fn solve_example() {
        assert_eq!(solve_part1(&parse_input(EXAMPLE_INPUT)), 136)
    }

    #[test]
    fn compare_slide_south_north() {
        let mut input = parse_input(EXAMPLE_INPUT);

        slide_rocks_north(&mut input.rocks);

        input.rocks.sort_by_key(|t| (t.loc.y, t.loc.x));
        let expected = input.clone();

        slide_rocks_south(&mut input.rocks, input.rows);
        slide_rocks_north(&mut input.rocks);

        input.rocks.sort_by_key(|t| (t.loc.y, t.loc.x));
        assert_eq!(input, expected);
    }

    #[test]
    fn compare_slide_east_west() {
        let mut input = parse_input(EXAMPLE_INPUT);

        slide_rocks_west(&mut input.rocks);

        input.rocks.sort_by_key(|t| (t.loc.y, t.loc.x));
        let expected = input.clone();

        slide_rocks_east(&mut input.rocks, input.cols);
        slide_rocks_west(&mut input.rocks);

        input.rocks.sort_by_key(|t| (t.loc.y, t.loc.x));
        assert_eq!(input, expected);
    }

    #[test]
    fn compare_slide_west_east() {
        let mut input = parse_input(EXAMPLE_INPUT);

        slide_rocks_east(&mut input.rocks, input.cols);

        input.rocks.sort_by_key(|t| (t.loc.y, t.loc.x));
        let expected = input.clone();

        println!("\n{:?}", input);

        slide_rocks_west(&mut input.rocks);
        slide_rocks_east(&mut input.rocks, input.cols);

        println!("\n{:?}", input);

        input.rocks.sort_by_key(|t| (t.loc.y, t.loc.x));
        assert_eq!(input, expected);
    }

    #[test]
    fn test_slide_rocks_east() {
        let mut input = parse_input(EXAMPLE_INPUT);
        slide_rocks_east(&mut input.rocks, input.cols);
        input.rocks.sort_by_key(|t| (t.loc.y, t.loc.x));

        assert_eq!(input, parse_input(EXAMPLE_RESULT_SLIDE_EAST));
    }

    #[test]
    fn test_after_1_cycle() {
        let mut input = parse_input(EXAMPLE_INPUT);

        cycle(&mut input.rocks, input.rows, input.cols);

        let mut expect = parse_input(AFTER_1_CYCLE);

        input.rocks.sort_by_key(|t| (t.loc.y, t.loc.x));
        expect.rocks.sort_by_key(|t| (t.loc.y, t.loc.x));

        assert_eq!(input, expect);
    }

    #[test]
    fn test_after_1_cycles() {
        let mut input = parse_input(EXAMPLE_INPUT);

        cycle(&mut input.rocks, input.rows, input.cols);
        cycle(&mut input.rocks, input.rows, input.cols);
        cycle(&mut input.rocks, input.rows, input.cols);

        let mut expect = parse_input(AFTER_3_CYCLES);

        input.rocks.sort_by_key(|t| (t.loc.y, t.loc.x));
        expect.rocks.sort_by_key(|t| (t.loc.y, t.loc.x));

        assert_eq!(input, expect);
    }

    #[test]
    fn solve_example_part2() {
        assert_eq!(solve_part2(&parse_input(EXAMPLE_INPUT)), 64)
    }
}
