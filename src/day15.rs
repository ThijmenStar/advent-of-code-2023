use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day15)]
pub fn parse_input(input: &str) -> Vec<String> {
    input.split(',').map(|s| String::from(s)).collect()
}

#[aoc(day15, part1)]
pub fn solve_part1(input: &[String]) -> u32 {
    input
        .iter()
        .map(|s| s.chars().fold(0, |val, c| ((val + c as u32) * 17) % 256))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn parse_example() {
        assert_eq!(
            parse_input(EXAMPLE_INPUT),
            vec![
                "rn=1", "cm-", "qp=3", "cm=2", "qp-", "pc=4", "ot=9", "ab=5", "pc-", "pc=6", "ot=7"
            ]
        )
    }

    #[test]
    fn solve_example_part1() {
        assert_eq!(solve_part1(&parse_input(EXAMPLE_INPUT)), 1320)
    }
}
