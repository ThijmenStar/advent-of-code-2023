use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;

#[derive(Debug, PartialEq)]
pub enum Tile {
    Start,
    // S
    Ground,
    // .
    Vertical,
    // |
    Horizontal,
    // -
    BendNE,
    // L
    BendNW,
    // J
    BendSW,
    // 7
    BendSE, // F
}

impl TryFrom<char> for Tile {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        use Tile::*;

        match value {
            'S' => Ok(Start),
            '.' => Ok(Ground),
            '|' => Ok(Vertical),
            '-' => Ok(Horizontal),
            'L' => Ok(BendNE),
            'J' => Ok(BendNW),
            '7' => Ok(BendSW),
            'F' => Ok(BendSE),
            _ => Err("Unknown tile"),
        }
    }
}

impl Tile {
    fn connects_to(&self) -> &[(i32, i32)] {
        match self {
            Tile::Start => &[(-1, 0), (0, 1), (1, 0), (0, -1)],
            Tile::Ground => &[],
            Tile::Vertical => &[(-1, 0), (1, 0)],
            Tile::Horizontal => &[(0, -1), (0, 1)],
            Tile::BendNE => &[(-1, 0), (0, 1)],
            Tile::BendNW => &[(-1, 0), (0, -1)],
            Tile::BendSW => &[(1, 0), (0, -1)],
            Tile::BendSE => &[(1, 0), (0, 1)],
        }
    }
}

#[aoc_generator(day10)]
pub fn parse_input(input: &str) -> Vec<Vec<Tile>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|tile| Tile::try_from(tile).expect("A valid tile"))
                .collect()
        })
        .collect()
}

#[aoc(day10, part1)]
pub fn solve_part1(grid: &Vec<Vec<Tile>>) -> u32 {
    let start_tile = grid
        .iter()
        .enumerate()
        .find_map(|(i, row)| {
            row.iter()
                .position(|tile| tile == &Tile::Start)
                .map(|j| (i, j))
        })
        .expect("Grid must contain start tile");

    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    visited.insert(start_tile);

    // Get pipes that connect to start
    let mut current_nodes: Vec<_> = [(-1, 0), (0, 1), (1, 0), (0, -1)]
        .into_iter()
        .map(|offset| {
            let new_pos = apply_offset(start_tile, offset);
            grid[new_pos.0][new_pos.1]
                .connects_to()
                .iter()
                .any(|connects_to| {
                    (offset.0 + connects_to.0) == 0 && (offset.1 + connects_to.1) == 0
                })
                .then_some(new_pos)
        })
        .flatten()
        .collect();

    for steps in 1.. {
        let mut next_nodes = vec![];
        for node in current_nodes.drain(..) {
            visited.insert(node);

            let pipe = &grid[node.0][node.1];
            for &offset in pipe.connects_to() {
                next_nodes.push(apply_offset(node, offset));
            }
        }

        next_nodes = next_nodes
            .into_iter()
            .filter(|node| !visited.contains(node))
            .collect();

        if next_nodes.is_empty() {
            return steps;
        } else {
            current_nodes = next_nodes;
        }
    }

    return 0;
}

fn apply_offset(pos: (usize, usize), offset: (i32, i32)) -> (usize, usize) {
    (
        (pos.0 as i32 + offset.0) as usize,
        (pos.1 as i32 + offset.1) as usize,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use Tile::*;

    const EXAMPLE_INPUT: &str = ".....
.S-7.
.|.|.
.L-J.
.....";

    #[test]
    fn parse_example() {
        assert_eq!(
            parse_input(EXAMPLE_INPUT),
            vec![
                vec![Ground, Ground, Ground, Ground, Ground],
                vec![Ground, Start, Horizontal, BendSW, Ground],
                vec![Ground, Vertical, Ground, Vertical, Ground],
                vec![Ground, BendNE, Horizontal, BendNW, Ground],
                vec![Ground, Ground, Ground, Ground, Ground],
            ]
        )
    }

    #[test]
    fn solve_example_part1() {
        assert_eq!(solve_part1(&parse_input(EXAMPLE_INPUT)), 4)
    }
}
