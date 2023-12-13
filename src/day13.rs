use aoc_runner_derive::{aoc, aoc_generator};
use std::fmt::{Debug, Formatter};

type Pattern = Vec<Vec<Tile>>;

#[derive(Eq, PartialEq, Clone)]
pub enum Tile {
    Ash,
    Rocks,
}

impl Debug for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Tile::Ash => ".",
                Tile::Rocks => "#",
            }
        )
    }
}

impl TryFrom<char> for Tile {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Tile::Ash),
            '#' => Ok(Tile::Rocks),
            _ => Err("Unknown tile"),
        }
    }
}

#[aoc_generator(day13)]
pub fn parse_input(input: &str) -> Vec<Pattern> {
    input
        .split("\n\n")
        .map(|pattern| {
            pattern
                .lines()
                .map(|line| {
                    line.chars()
                        .map(|tile| Tile::try_from(tile).unwrap())
                        .collect()
                })
                .collect()
        })
        .collect()
}

#[aoc(day13, part1)]
pub fn solve_part1(patterns: &[Pattern]) -> usize {
    let vertical_sum: usize = patterns
        .iter()
        .map(|pattern| {
            splits(pattern)
                .into_iter()
                .map(|(a, b)| match is_mirrored(a, b) {
                    true => a.len(),
                    false => 0,
                })
                .sum::<usize>()
        })
        .sum();

    let horizontal_sum: usize = patterns
        .iter()
        .map(|pattern| {
            splits(&transpose(pattern))
                .into_iter()
                .map(|(a, b)| match is_mirrored(a, b) {
                    true => a.len(),
                    false => 0,
                })
                .sum::<usize>()
        })
        .sum();

    100 * vertical_sum + horizontal_sum
}

#[aoc(day13, part2)]
pub fn solve_part2(patterns: &[Pattern]) -> usize {
    let vertical_sum: usize = patterns
        .iter()
        .map(|pattern| {
            splits(pattern)
                .into_iter()
                .map(|(a, b)| match is_mirrored_smudge(a, b) {
                    true => a.len(),
                    false => 0,
                })
                .sum::<usize>()
        })
        .sum();

    let horizontal_sum: usize = patterns
        .iter()
        .map(|pattern| {
            splits(&transpose(pattern))
                .into_iter()
                .map(|(a, b)| match is_mirrored_smudge(a, b) {
                    true => a.len(),
                    false => 0,
                })
                .sum::<usize>()
        })
        .sum();

    100 * vertical_sum + horizontal_sum
}

fn transpose<T>(v: &[Vec<T>]) -> Vec<Vec<T>>
where
    T: Clone,
{
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}

fn splits<T>(iter: &[T]) -> Vec<(&[T], &[T])> {
    (1..iter.len())
        .map(|split_index| {
            let stuff = iter.split_at(split_index);
            stuff
        })
        .collect()
}

fn is_mirrored(a: &[Vec<Tile>], b: &[Vec<Tile>]) -> bool {
    a.iter().rev().zip(b).all(|(row_a, row_b)| row_a == row_b)
}

fn is_mirrored_smudge(a: &[Vec<Tile>], b: &[Vec<Tile>]) -> bool {
    match a.iter().rev().zip(b).map(|(row_a, row_b)|
        row_a.iter().zip(row_b).filter(|(i, j)| i != j).count()
    ).sum() {
        1 => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use Tile::*;

    const EXAMPLE_INPUT: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

    #[test]
    fn test_is_mirrored() {
        assert_eq!(
            is_mirrored(
                vec![vec![Rocks, Ash, Rocks, Ash]].as_slice(),
                vec![vec![Rocks, Ash, Rocks, Ash]].as_slice(),
            ),
            true
        );

        assert_eq!(
            is_mirrored(
                vec![vec![Rocks, Ash, Rocks, Rocks]].as_slice(),
                vec![vec![Rocks, Ash, Rocks, Ash]].as_slice(),
            ),
            false
        );

        assert_eq!(
            is_mirrored(
                vec![vec![Ash, Rocks, Ash, Rocks], vec![Rocks, Ash, Rocks, Ash],].as_slice(),
                vec![vec![Rocks, Ash, Rocks, Ash]].as_slice(),
            ),
            true
        );

        assert_eq!(
            is_mirrored(
                vec![vec![Ash, Rocks, Ash, Rocks]].as_slice(),
                vec![vec![Ash, Rocks, Ash, Rocks], vec![Rocks, Ash, Rocks, Ash],].as_slice()
            ),
            true
        );

        assert_eq!(
            is_mirrored(
                vec![vec![Rocks, Ash, Rocks, Ash], vec![Ash, Rocks, Ash, Rocks],].as_slice(),
                vec![vec![Ash, Rocks, Ash, Rocks], vec![Rocks, Ash, Rocks, Ash],].as_slice()
            ),
            true
        );
    }

    #[test]
    fn test_split_iter() {
        let stuff = &[1, 2, 3, 4];

        assert_eq!(
            splits(stuff),
            vec![
                (&[1][..], &[2, 3, 4][..]),
                (&[1, 2][..], &[3, 4][..]),
                (&[1, 2, 3][..], &[4][..]),
            ]
        );
    }

    #[test]
    fn test_transpose() {
        let matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];

        let transposed_matrix = transpose(&matrix);

        let expected_transposed_matrix = vec![vec![1, 4, 7], vec![2, 5, 8], vec![3, 6, 9]];

        assert_eq!(transposed_matrix, expected_transposed_matrix);
    }

    #[test]
    fn solve_example_part1() {
        assert_eq!(solve_part1(&parse_input(EXAMPLE_INPUT)), 405)
    }

    #[test]
    fn solve_example_part2() {
        assert_eq!(solve_part2(&parse_input(EXAMPLE_INPUT)), 400)
    }
}
