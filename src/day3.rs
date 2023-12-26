use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;

type Coordinate = (usize, usize);
type Engine = Vec<Vec<char>>;

#[aoc_generator(day3)]
fn parse_input(input: &str) -> Engine {
    input.lines().map(|x| x.chars().collect()).collect()
}

fn is_symbol(c: char) -> bool {
    !c.is_ascii_digit() && c != '.'
}

fn is_adjacent(a: Coordinate, b: Coordinate) -> bool {
    a.0.abs_diff(b.0) <= 1 && a.1.abs_diff(b.1) <= 1
}

fn get_symbol_coordinates(engine: &Engine) -> Vec<Coordinate> {
    engine
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.iter()
                .enumerate()
                .filter(|(_, &c)| is_symbol(c))
                .map(move |(x, _)| (x, y))
        })
        .collect()
}

fn get_gear_coordinates(engine: &Engine) -> Vec<Coordinate> {
    engine
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.iter()
                .enumerate()
                .filter(|(_, &c)| c == '*')
                .map(move |(x, _)| (x, y))
        })
        .collect()
}

#[aoc(day3, part1)]
fn solve_part1(engine: &Engine) -> u32 {
    let symbols = get_symbol_coordinates(engine);

    let mut solution = 0;

    for y in 0..engine.len() {
        let mut acc = 0;
        let mut near_symbol = false;

        for x in 0..engine[0].len() {
            match engine[y][x].to_digit(10) {
                Some(d) => {
                    acc = 10 * acc + d;
                    if !near_symbol {
                        near_symbol = symbols.iter().any(|s| is_adjacent((x, y), *s))
                    }
                }
                None => {
                    if acc > 0 && near_symbol {
                        solution += acc;
                    }
                    near_symbol = false;
                    acc = 0;
                }
            }
        }

        if acc > 0 && near_symbol {
            solution += acc;
        }
    }

    return solution;
}

#[aoc(day3, part2)]
fn solve_part2(engine: &Engine) -> u32 {
    let gears = get_gear_coordinates(engine);
    let mut gears_numbers: Vec<Vec<u32>> = vec![vec![]; gears.len()];

    for y in 0..engine.len() {
        let mut acc = 0;
        let mut near_gears = HashSet::new();

        for x in 0..engine[0].len() {
            match engine[y][x].to_digit(10) {
                Some(d) => {
                    acc = 10 * acc + d;
                    near_gears.extend(
                        gears
                            .iter()
                            .enumerate()
                            .filter(|(_, s)| is_adjacent((x, y), **s))
                            .map(|(i, _)| i)
                    );
                }
                None => {
                    if acc > 0 && !near_gears.is_empty() {
                        for i in near_gears {
                            gears_numbers[i].push(acc);
                        }
                    }
                    near_gears = HashSet::new();
                    acc = 0;
                }
            };

        }

        if acc > 0 && !near_gears.is_empty() {
            for i in near_gears {
                gears_numbers[i].push(acc);
            }
        }
    }

    return gears_numbers
        .iter()
        .filter(|numbers| numbers.len() == 2)
        .map(|numbers| numbers[0]*numbers[1])
        .sum();
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_ENGINE: &str = "467..114..\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598..";

    #[test]
    fn solve_example_part_1() {
        assert_eq!(solve_part1(&parse_input(EXAMPLE_ENGINE)), 4361);
    }

    #[test]
    fn rando_test_reddit() {
        assert_eq!(solve_part1(&parse_input("........\n.24..4..\n......*.")), 4);
    }

    #[test]
    fn own_test() {
        let input = "1...2\n.*.*.\n.....\n3...4";

        assert_eq!(solve_part1(&parse_input(input)), 3);
    }

    #[test]
    fn test_is_symbol() {
        // symbols
        assert!(is_symbol('&'));
        assert!(is_symbol('='));
        assert!(is_symbol('/'));
        assert!(is_symbol('@'));
        assert!(is_symbol('$'));

        // not symbols
        assert!(!is_symbol('.'));
        assert!(!is_symbol('5'));
    }

    #[test]
    fn solve_example_part_2() {
        assert_eq!(solve_part2(&parse_input(EXAMPLE_ENGINE)), 467835);
    }
}
