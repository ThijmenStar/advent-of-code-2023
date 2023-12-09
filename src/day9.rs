use aoc_runner_derive::{aoc, aoc_generator};


#[aoc_generator(day9)]
pub fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse().expect("A valid number"))
                .collect()
        })
        .collect()
}



#[aoc(day9, part1)]
pub fn solve_part1(input: &Vec<Vec<i32>>) -> i32 {

    input
        .iter()
        .map(|numbers| extrapolate(numbers))
        .sum()

}


fn extrapolate(numbers: &Vec<i32>) -> i32 {
    if numbers.iter().all(|&n| n == 0) {
        0
    } else {
        numbers.last().expect("Cannot extrapolate empty vec") + extrapolate(&differences(numbers))
    }
}

fn differences(numbers: &Vec<i32>) -> Vec<i32> {
    numbers.
        iter()
        .zip(
            numbers.iter().skip(1)
        )
        .map(|(a, b)| b-a)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn parse_example() {
        assert_eq!(
            parse_input(EXAMPLE_INPUT),
            vec![
                vec![0, 3, 6, 9, 12, 15],
                vec![1, 3, 6, 10, 15, 21],
                vec![10, 13, 16, 21, 30, 45],
            ]
        )
    }

    #[test]
    fn test_vec_differences() {
        assert_eq!(differences(&parse_input(EXAMPLE_INPUT)[0]), vec![3, 3, 3, 3, 3])
    }


    #[test]
    fn test_extrapolate() {
        assert_eq!(extrapolate(&vec![0, 0, 0]), 0)
    }
}
