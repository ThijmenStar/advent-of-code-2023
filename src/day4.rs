use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;

#[derive(PartialEq, Debug)]
struct Card {
    id: u32,
    winning_numbers: Vec<u32>,
    numbers_you_have: Vec<u32>,
}

#[aoc_generator(day4)]
fn parse_input(input: &str) -> Vec<Card> {
    input
        .lines()
        .map(|line| {
            let (metadata, data) = line.split_once(':').unwrap();
            let id = metadata.split_whitespace().last().unwrap().parse().unwrap();
            let (win_nums, have_nums) = data.split_once('|').unwrap();

            let winning_numbers = win_nums
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect();
            let numbers_you_have = have_nums
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect();

            Card {
                id,
                winning_numbers,
                numbers_you_have,
            }
        })
        .collect()
}

fn calculate_points(matches: u32) -> u32 {
    if matches == 0 {
        0
    } else {
        2_u32.pow(matches - 1)
    }
}

#[aoc(day4, part1)]
fn solve_part1(input: &[Card]) -> u32 {
    input
        .iter()
        .map(|card| {
            let winning: HashSet<u32> = HashSet::from_iter(card.winning_numbers.iter().cloned());
            let having: HashSet<u32> = HashSet::from_iter(card.numbers_you_have.iter().cloned());
            calculate_points(winning.intersection(&having).count() as u32)
        })
        .sum()
}

#[aoc(day4, part2)]
fn solve_part2(input: &[Card]) -> usize {
    let matching_numbers: Vec<usize> = input
        .iter()
        .map(|card| {
            let winning: HashSet<u32> = HashSet::from_iter(card.winning_numbers.iter().cloned());
            let having: HashSet<u32> = HashSet::from_iter(card.numbers_you_have.iter().cloned());
            winning.intersection(&having).count()
        })
        .collect();

    let mut copies: Vec<usize> = vec![1; matching_numbers.len()];

    for i in 0..matching_numbers.len() {
        let new_copies = matching_numbers[i];

        // add copies
        for j in i + 1..i + 1 + new_copies {
            copies[j] += copies[i];
        }
    }

    return copies.iter().sum();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        const EXAMPLE_INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";

        assert_eq!(
            parse_input(EXAMPLE_INPUT),
            vec![Card {
                id: 1,
                winning_numbers: vec![41, 48, 83, 86, 17],
                numbers_you_have: vec![83, 86, 6, 31, 17, 9, 48, 53]
            }]
        )
    }

    #[test]
    fn test_calculate_points() {
        assert_eq!(calculate_points(0), 0);
        assert_eq!(calculate_points(1), 1);
        assert_eq!(calculate_points(2), 2);
        assert_eq!(calculate_points(3), 4);
        assert_eq!(calculate_points(4), 8);
    }

    #[test]
    fn solve_example_part1() {
        const EXAMPLE_INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        assert_eq!(solve_part1(&parse_input(EXAMPLE_INPUT)), 13)
    }

    #[test]
    fn solve_example_part2() {
        const EXAMPLE_INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        assert_eq!(solve_part2(&parse_input(EXAMPLE_INPUT)), 30)
    }
}
