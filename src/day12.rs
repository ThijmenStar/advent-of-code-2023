use std::collections::HashMap;
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
pub enum Condition {
    Operational,
    Damaged,
    Unknown,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Record {
    conditions: Vec<Condition>,
    damaged_springs: Vec<usize>,
}

#[aoc_generator(day12)]
pub fn parse_input(input: &str) -> Vec<Record> {
    input
        .lines()
        .map(|line| {
            let (conditions, damaged_springs) = line.split_once(' ').expect("Failed to split line");

            Record {
                conditions: conditions
                    .chars()
                    .map(|c| match c {
                        '.' => Condition::Operational,
                        '#' => Condition::Damaged,
                        '?' => Condition::Unknown,
                        _ => panic!("Invalid character"),
                    })
                    .collect(),
                damaged_springs: damaged_springs
                    .split(',')
                    .map(|n| n.parse().expect("A valid numer"))
                    .collect(),
            }
        })
        .collect()
}

#[aoc(day12, part1)]
pub fn solve_part1(report: &[Record]) -> usize {
    report
        .iter()
        .map(|record| {
            solver(
                &record
                    .conditions
                    .clone()
                    .into_iter()
                    .enumerate()
                    .filter(|(_, condition)| {
                        condition == &Condition::Damaged || condition == &Condition::Unknown
                    })
                    .collect::<Vec<_>>(),
                &record.damaged_springs,
                &mut HashMap::new()
            )
        })
        .sum()
}

#[aoc(day12, part2)]
pub fn solve_part2(report: &[Record]) -> usize {
    report
        .iter()
        .map(|record| {
            let temp: Vec<Vec<Condition>> =
                std::iter::repeat(record.conditions.clone()).take(5).collect();
            Record {
                conditions: temp.join(&Condition::Unknown),
                damaged_springs: record.damaged_springs.repeat(5),
            }
        })
        .map(|record| {
            solver(
                &record
                    .conditions
                    .clone()
                    .into_iter()
                    .enumerate()
                    .filter(|(_, condition)| {
                        condition == &Condition::Damaged || condition == &Condition::Unknown
                    })
                    .collect::<Vec<_>>(),
                &record.damaged_springs,
                &mut HashMap::new()
            )
        })
        .sum()
}

fn solver(conditions: &[(usize, Condition)], groups: &[usize], cache: &mut HashMap<(usize, usize), usize>) -> usize {
    if conditions.is_empty() {
        // base case
        return if groups.is_empty() { 1 } else { 0 };
    }
    let current_group_size = match groups.first() {
        Some(s) => *s,
        None => {
            return conditions
                .iter()
                .all(|(_, c)| c == &Condition::Unknown)
                .then_some(1)
                .unwrap_or(0)
        }
    };

    let (index, current_condition) = conditions.first().unwrap();

    match cache.get(&(*index, groups.len())) {
        Some(&result) => result,
        None => {
            let result = match (index, current_condition) {
                (i, Condition::Damaged) => consume(&conditions, &groups, current_group_size, i, cache),
                (i, Condition::Unknown) => {
                    let consumed_sum = consume(&conditions, &groups, current_group_size, i, cache);
                    let skipped_sum = solver(&conditions[1..], &groups[..], cache);
                    consumed_sum + skipped_sum
                }
                _ => panic!("Conditions should only contain Damaged and Unknown"),
            };
            cache.insert((*index, groups.len()), result);
            result
        }
    }
}

fn consume(
    conditions: &&[(usize, Condition)],
    groups: &&[usize],
    current_group_size: usize,
    i: &usize,
    cache: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if conditions.len() < current_group_size {
        // cannot consume
        return 0;
    }

    if is_continuous(&conditions[0..current_group_size]) {
        // check not followed by Condition
        match conditions.get(current_group_size) {
            None => solver(&conditions[current_group_size..], &groups[1..], cache), // no more conditions
            Some((j, _)) if j > &(i + &current_group_size) => {
                // not followed
                solver(&conditions[current_group_size..], &groups[1..], cache)
            }
            Some((j, Condition::Unknown)) if j == &(i + current_group_size) => {
                // followed by an unknown
                solver(&conditions[current_group_size + 1..], &groups[1..], cache)
            }
            _ => 0,
        }
    } else {
        0
    }
}

fn is_continuous(conditions: &[(usize, Condition)]) -> bool {
    conditions
        .iter()
        .tuple_windows()
        .map(|((a, _), (b, _))| b - a)
        .all(|delta| delta == 1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use Condition::Damaged as D;
    use Condition::Operational as O;
    use Condition::Unknown as U;

    const EXAMPLE_INPUT: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

    #[test]
    fn parse_example_input() {
        assert_eq!(
            parse_input(EXAMPLE_INPUT)[0],
            Record {
                conditions: vec![U, U, U, O, D, D, D],
                damaged_springs: vec![1, 1, 3],
            },
        )
    }

    #[test]
    fn solve_example_part1() {
        assert_eq!(solve_part1(&parse_input(EXAMPLE_INPUT)), 21)
    }

    #[test]
    fn solve_example_part2() {
        assert_eq!(solve_part2(&parse_input(EXAMPLE_INPUT)), 525152)
    }

    #[test]
    fn test_solve() {
        // base cases
        assert_eq!(
            solver(&vec![], &vec![1, 1, 1], &mut HashMap::new()),
            0,
            "Empty conditions should return None when not all groups are consumed"
        );
        assert_eq!(
            solver(&vec![], &vec![], &mut HashMap::new()),
            1,
            "Empty conditions should return one when all groups are consumed"
        );

        // Only knows
        assert_eq!(
            solver(&vec![(1, D), (4, D), (7, D)], &vec![1, 1, 1], &mut HashMap::new()),
            1
        );

        assert_eq!(solver(&vec![(1, D), (2, D), (7, D)], &vec![1, 1, 1], &mut HashMap::new()), 0);

        assert_eq!(solver(&vec![(1, D), (2, U), (7, D)], &vec![1, 1], &mut HashMap::new()), 1);

        assert_eq!(solver(&vec![(1, D), (7, D), (8, U)], &vec![1, 1], &mut HashMap::new()), 1);

        assert_eq!(
            solver(
                &vec![(1, U), (2, U), (5, U), (6, U), (10, U), (11, D), (12, D)],
                &vec![1, 1, 3], &mut HashMap::new()
            ),
            4
        );

        assert_eq!(
            solver(&vec![(4, U)], &vec![1, 2], &mut HashMap::new()),
            0
        );

        assert_eq!(
            solver(
                vec![U, U, U, U, U]
                    .into_iter()
                    .enumerate()
                    .collect::<Vec<_>>()
                    .as_slice(),
                &vec![1, 2],
                &mut HashMap::new()
            ),
            3
        );


        assert_eq!(solver(&vec![(1, U), (2, U), (3, U)], &vec![1], &mut HashMap::new()), 3);

        assert_eq!(
            solver(
                vec![U, D, D, D, U, U, U, U, U, U, U, U]
                    .into_iter()
                    .enumerate()
                    .collect::<Vec<_>>()
                    .as_slice(),
                &vec![3, 2, 1],
                    &mut HashMap::new()
            ),
            10
        );
    }

    #[test]
    fn test_is_continuous() {
        assert_eq!(is_continuous(&vec![(1, D), (2, U), (3, D)]), true);
        assert_eq!(
            is_continuous(&vec![(1, D), (2, U), (3, D), (5, U), (7, D)]),
            false
        );
    }
}
