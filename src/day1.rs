use std::collections::HashMap;
use aoc_runner_derive::aoc;
use regex::Regex;

#[aoc(day1, part1)]
fn solve_part1(input: &str) -> u32 {
    let mut answer = 0;

    let digit_hashmap = HashMap::from([
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
    ]);

    let re_first = Regex::new(r"(?<digit>\d).*").unwrap();
    let re_last = Regex::new(r".*(?<digit>\d)").unwrap();

    for l in input.lines() {
        let caps_first = re_first.captures(l).unwrap();
        let caps_last = re_last.captures(l).unwrap();

        let calibration_value = digit_hashmap.get(&caps_first["digit"]).unwrap() * 10 + digit_hashmap.get(&caps_last["digit"]).unwrap();
        answer += calibration_value;
    }

    return answer;
}
#[aoc(day1, part2)]
fn solve_part2(input: &str) -> u32 {
    let mut answer = 0;

    let digit_hashmap = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
    ]);

    let re_first = Regex::new(r"(?<digit>\d|one|two|three|four|five|six|seven|eight|nine).*").unwrap();
    let re_last = Regex::new(r".*(?<digit>\d|one|two|three|four|five|six|seven|eight|nine)").unwrap();

    for l in input.lines() {
        let caps_first = re_first.captures(l).unwrap();
        let caps_last = re_last.captures(l).unwrap();

        let calibration_value = digit_hashmap.get(&caps_first["digit"]).unwrap() * 10 + digit_hashmap.get(&caps_last["digit"]).unwrap();
        answer += calibration_value;
    }

    return answer;
}