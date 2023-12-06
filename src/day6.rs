use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

type Race = (u64, u64);

#[aoc_generator(day6, part1)]
pub fn parse_input_part1(input: &str) -> Vec<Race> {

    let lines: Vec<&str> = input.lines().collect();
    let times: Vec<u64> = lines[0].split_whitespace().skip(1).map(|i| i.parse().unwrap()).collect();
    let distances: Vec<u64> = lines[1].split_whitespace().skip(1).map(|i| i.parse().unwrap()).collect();

    times.into_iter().zip(distances).collect()
}

#[aoc_generator(day6, part2)]
pub fn parse_input_part2(input: &str) -> Race {

    let lines: Vec<&str> = input.lines().collect();
    let time: u64 = lines[0].split_whitespace().skip(1).join("").parse().unwrap();
    let distance: u64 = lines[1].split_whitespace().skip(1).join("").parse().unwrap();

    (time, distance)
}

#[aoc(day6, part1)]
pub fn solve_part1(input: &[Race]) -> u64 {
    input.iter().map(|&(time, distance)| {
        ways_to_beat_race(time, distance)

    }).product()
}

fn ways_to_beat_race(time: u64, distance: u64) -> u64 {
    let x0 = ((time as f64 - ((time as f64).powi(2) - 4.0 * ((distance as f64) + 1.0)).sqrt()) / 2.0).ceil() as u64;
    let x1 = ((time as f64 + ((time as f64).powi(2) - 4.0 * ((distance as f64) + 1.0)).sqrt()) / 2.0).floor() as u64;

    x1 - x0 + 1
}

#[aoc(day6, part2)]
pub fn solve_part2(input: &Race) -> u64 {
    let (time, distance) = *input;

    ways_to_beat_race(time, distance)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "Time:      7  15   30\nDistance:  9  40  200";

    #[test]
    fn parse_example_input_part1() {

        assert_eq!(parse_input_part1(EXAMPLE_INPUT), vec![(7, 9), (15, 40), (30, 200)])

    }

    #[test]
    fn parse_example_input_part2() {

        assert_eq!(parse_input_part2(EXAMPLE_INPUT), (71530, 940200))

    }

    #[test]
    fn solve_example_part1() {
        assert_eq!(solve_part1(&parse_input_part1(EXAMPLE_INPUT)), 288);
    }

    #[test]
    fn solve_example_part2() {
        assert_eq!(solve_part2(&parse_input_part2(EXAMPLE_INPUT)), 71503);
    }
}
