use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day15)]
pub fn parse_input(input: &str) -> Vec<String> {
    input.split(',').map(|s| String::from(s)).collect()
}

#[aoc(day15, part1)]
pub fn solve_part1(input: &[String]) -> usize {
    input.iter().map(|s| hash(s)).sum()
}

#[aoc(day15, part2)]
pub fn solve_part2(input: &[String]) -> usize {
    let mut boxes: Vec<Vec<(&str, u8)>> = vec![vec![]; 256];

    input.iter().for_each(|s| match s.chars().last() {
        Some('-') => {
            let label = &s[..s.len() - 1];
            let index = hash(label);
            boxes[index]
                .iter()
                .position(|(l, _)| &label == l)
                .map(|pos| boxes[index].remove(pos));
        }
        _ => {
            let (label, lens) = s.split_once('=').unwrap();
            let index = hash(label);
            let focal: u8 = lens.parse().unwrap();
            boxes[index]
                .iter()
                .position(|(l, _)| &label == l)
                .map(|pos| boxes[index][pos] = (label, focal))
                .unwrap_or_else(|| boxes[index].push((label, focal)))
        }
    });

    boxes
        .iter()
        .enumerate()
        .flat_map(|(i, b)| {
            b.iter()
                .enumerate()
                .map(move |(j, &(_, lens))| (i + 1) * (j + 1) * (lens as usize))
        })
        .sum()
}

fn hash(s: &str) -> usize {
    s.chars().fold(0, |val, c| ((val + c as usize) * 17) % 256)
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

    #[test]
    fn solve_example_part2() {
        assert_eq!(solve_part2(&parse_input(EXAMPLE_INPUT)), 145)
    }
}
