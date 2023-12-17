use aoc_runner_derive::{aoc, aoc_generator};
use glam::IVec2;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

#[derive(Debug)]
struct Map {
    nodes: HashMap<IVec2, usize>,

    rows: usize,
    cols: usize,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn left(&self) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        }
    }

    fn right(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    fn straight(&self) -> Direction {
        *self
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Heading {
    direction: Direction,
    straight_for: u8,
}

impl Heading {
    fn next_headings(&self, min_straight: u8, max_straight: u8) -> Vec<Heading> {
        if self.straight_for < min_straight {
            vec![Heading {
                direction: self.direction.straight(),
                straight_for: self.straight_for + 1,
            }]
        } else if self.straight_for < max_straight {
            vec![
                Heading {
                    direction: self.direction.left(),
                    straight_for: 1,
                },
                Heading {
                    direction: self.direction.straight(),
                    straight_for: self.straight_for + 1,
                },
                Heading {
                    direction: self.direction.right(),
                    straight_for: 1,
                },
            ]
        } else {
            vec![
                Heading {
                    direction: self.direction.left(),
                    straight_for: 1,
                },
                Heading {
                    direction: self.direction.right(),
                    straight_for: 1,
                },
            ]
        }
    }

    fn next_position(&self, current_position: IVec2) -> IVec2 {
        match self.direction {
            Direction::North => current_position + IVec2::new(0, -1),
            Direction::East => current_position + IVec2::new(1, 0),
            Direction::South => current_position + IVec2::new(0, 1),
            Direction::West => current_position + IVec2::new(-1, 0),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: IVec2,
    heading: Heading,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
        // maybe have to handle ties here
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[aoc_generator(day17)]
fn parse_input(input: &str) -> Map {
    Map {
        nodes: input
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars().enumerate().map(move |(x, d)| {
                    (
                        IVec2::new(x as i32, y as i32),
                        d.to_digit(10).unwrap() as usize,
                    )
                })
            })
            .collect(),
        rows: input.lines().count(),
        cols: input.lines().next().unwrap().len(),
    }
}

#[aoc(day17, part1)]
fn solve_part1(input: &Map) -> usize {
    const MIN_STRAIGHT: u8 = 0;
    const MAX_STRAIGHT: u8 = 3;

    return solve(input, MIN_STRAIGHT, MAX_STRAIGHT).expect("path should exist");
}

#[aoc(day17, part2)]
fn solve_part2(input: &Map) -> usize {
    const MIN_STRAIGHT: u8 = 4;
    const MAX_STRAIGHT: u8 = 10;

    return solve(input, MIN_STRAIGHT, MAX_STRAIGHT).expect("path should exist");
}

fn solve(input: &Map, min_straight: u8, max_straight: u8) -> Option<usize> {
    let mut dist: HashMap<(IVec2, Heading), usize> = HashMap::new();
    let mut heap = BinaryHeap::new();

    let goal = IVec2::new(input.cols as i32 - 1, input.rows as i32 - 1);

    heap.push(State {
        cost: 0,
        position: IVec2::ZERO,
        heading: Heading {
            direction: Direction::East,
            straight_for: 0,
        },
    });
    heap.push(State {
        cost: 0,
        position: IVec2::ZERO,
        heading: Heading {
            direction: Direction::South,
            straight_for: 0,
        },
    });

    while let Some(State {
        cost,
        position,
        heading,
    }) = heap.pop()
    {
        // Shortest path found
        if position == goal && heading.straight_for >= min_straight {
            return Some(cost);
        }

        if &cost > dist.get(&(position, heading)).unwrap_or(&usize::MAX) {
            continue; // We already found a better way
        }

        for next_heading in heading.next_headings(min_straight, max_straight) {
            let next_position = next_heading.next_position(position);
            if next_position.cmplt(IVec2::ZERO).any() || next_position.cmpgt(goal).any() {
                continue; // Don't explore off grid
            }

            let next = State {
                cost: cost + input.nodes.get(&next_position).unwrap(),
                position: next_position,
                heading: next_heading,
            };

            if &next.cost
                < dist
                    .get(&(next.position, next.heading))
                    .unwrap_or(&usize::MAX)
            {
                heap.push(next);

                // We found a better way
                dist.insert((next.position, next.heading), next.cost);
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;

    const EXAMPLE_INPUT: &str = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";

    const ANOTHER_EXAMPLE: &str = "111111111111
999999999991
999999999991
999999999991
999999999991";

    #[test]
    fn test_solve_example_part1() {
        assert_eq!(solve_part1(&parse_input(EXAMPLE_INPUT)), 102);
    }

    #[test]
    fn test_solve_example_part2() {
        assert_eq!(solve_part2(&parse_input(EXAMPLE_INPUT)), 94);
    }
    #[test]
    fn test_solve_another_example_part2() {
        assert_eq!(solve_part2(&parse_input(ANOTHER_EXAMPLE)), 71);
    }

    #[test]
    fn test_boundaries_horizontal() {
        assert_eq!(solve(&parse_input("1111"), 4, 10), None);
        assert_eq!(solve(&parse_input("11111"), 4, 10), Some(4));
        assert_eq!(solve(&parse_input("1111111111"), 4, 10), Some(9));
        assert_eq!(solve(&parse_input("11111111111"), 4, 10), Some(10));
        assert_eq!(solve(&parse_input("111111111111"), 4, 10), None);
    }

    #[test]
    fn test_boundaries_vertical() {
        assert_eq!(
            solve(&parse_input("1111".chars().join("\n").as_str()), 4, 10),
            None
        );
        assert_eq!(
            solve(&parse_input("11111".chars().join("\n").as_str()), 4, 10),
            Some(4)
        );
        assert_eq!(
            solve(
                &parse_input("1111111111".chars().join("\n").as_str()),
                4,
                10
            ),
            Some(9)
        );
        assert_eq!(
            solve(
                &parse_input("11111111111".chars().join("\n").as_str()),
                4,
                10
            ),
            Some(10)
        );
        assert_eq!(
            solve(
                &parse_input("111111111111".chars().join("\n").as_str()),
                4,
                10
            ),
            None
        );
    }
}
