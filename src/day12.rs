use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[derive(Debug, Eq, PartialEq)]
pub enum Condition {
    Operational,
    Damaged,
    Unknown,
}

#[derive(Debug, Eq, PartialEq)]
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
            let unknowns: Vec<usize> = record
                .conditions
                .iter()
                .positions(|c| c == &Condition::Unknown)
                .collect();

            let damaged: Vec<usize> = record
                .conditions
                .iter()
                .positions(|c| c == &Condition::Damaged)
                .collect();

            let unknown_damaged = record.damaged_springs.iter().sum::<usize>() - damaged.len();

            unknowns
                .into_iter()
                .combinations(unknown_damaged)
                .filter(|combination| {
                    // dbg!(combination, &damaged);
                    validate_report(&combination, &damaged, &record.damaged_springs)
                })
                .count()
        })
        .sum()
}

fn validate_report(
    unknown_damaged: &[usize],
    known_damaged: &[usize],
    damaged_springs: &[usize],
) -> bool {
    let mut counts: Vec<usize> = vec![];
    [0].iter()
        .chain(unknown_damaged)
        .chain(known_damaged)
        .sorted()
        .tuple_windows()
        .for_each(|(previous, current)| {
            if current - previous == 1 {
                match counts.last_mut() {
                    Some(count) => *count += 1,
                    None => counts.push(1),
                }
            } else {
                counts.push(1);
            }
        });

    return counts == damaged_springs;
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

    #[test]
    fn parse_example_input() {
        use Condition::Damaged as D;
        use Condition::Operational as O;
        use Condition::Unknown as U;

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
}
