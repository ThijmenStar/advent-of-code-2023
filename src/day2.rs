use aoc_runner_derive::{aoc, aoc_generator};

#[derive(PartialEq, Debug)]
pub struct Game {
   id: u32,
}

#[aoc_generator(day2)]
pub fn parse_input(input: &str) -> Vec<Game> {
    let game = Game {id: 1};
    println!("{}", input);

    return Vec::from([game])
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &[Game]) -> u32 {

    return 0;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(parse_input("kaas"), Vec::new())
    }

}