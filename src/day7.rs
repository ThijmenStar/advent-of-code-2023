use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::cmp::Ordering;

#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Debug, Clone)]
enum Card {
    Ace,
    King,
    Queen,
    Jack,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
    Joker,
}

impl TryFrom<char> for Card {
    type Error = &'static str;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'A' => Ok(Card::Ace),
            'K' => Ok(Card::King),
            'Q' => Ok(Card::Queen),
            'J' => Ok(Card::Jack),
            'T' => Ok(Card::Ten),
            '9' => Ok(Card::Nine),
            '8' => Ok(Card::Eight),
            '7' => Ok(Card::Seven),
            '6' => Ok(Card::Six),
            '5' => Ok(Card::Five),
            '4' => Ok(Card::Four),
            '3' => Ok(Card::Three),
            '2' => Ok(Card::Two),
            '*' => Ok(Card::Joker),
            _ => Err("Card not recognized"),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone)]
enum CardsType {
    FiveOKind,
    FourOKind,
    FullHouse,
    ThreeOKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(Eq, Debug, Clone)]
pub struct Hand {
    cards: Vec<Card>,
    bid: usize,
}

impl Hand {
    fn get_type(&self) -> CardsType {
        let mut counts = self.cards.iter().counts();

        let jokers = counts.remove(&Card::Joker).unwrap_or(0);

        let occurrences = counts.values().sorted().rev().collect::<Vec<&usize>>();

        match occurrences[..] {
            [] if jokers == 5 => CardsType::FiveOKind,
            [x] if x + jokers == 5 => CardsType::FiveOKind,
            [x, ..] if x + jokers == 4 => CardsType::FourOKind,
            [x, y] if x + y + jokers == 5 => CardsType::FullHouse,
            [x, ..] if x + jokers == 3 => CardsType::ThreeOKind,
            [x, y, ..] if x + y + jokers == 4 => CardsType::TwoPair,
            [x, ..] if x + jokers == 2 => CardsType::OnePair,
            [..] => CardsType::HighCard,
        }
    }
}

impl TryFrom<&str> for Hand {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (cards, bid) = value.split_once(' ').ok_or("Failed to split")?;
        Ok(Hand {
            cards: cards.chars().map(|c| Card::try_from(c)).try_collect()?,
            bid: bid.parse().map_err(|_| "Failed to parse bid")?,
        })
    }
}
impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.get_type().cmp(&other.get_type()) {
            Ordering::Less => Ordering::Less,
            Ordering::Equal => self.cards.cmp(&other.cards),
            Ordering::Greater => Ordering::Greater,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

#[aoc_generator(day7, part1)]
pub fn parse_input_part1(input: &str) -> Vec<Hand> {
    input
        .lines()
        .map(|line| Hand::try_from(line).expect("A valid hand"))
        .collect()
}

#[aoc_generator(day7, part2)]
pub fn parse_input_part2(input: &str) -> Vec<Hand> {
    input
        .lines()
        .map(|line| Hand::try_from(line.replace('J', "*").as_str()).expect("A valid hand"))
        .collect()
}

#[aoc(day7, part1)]
pub fn solve_part1(input: &[Hand]) -> usize {
    input
        .iter()
        .sorted()
        .rev()
        .enumerate()
        .map(|(i, hand)| (i + 1) * hand.bid)
        .sum()
}

#[aoc(day7, part2)]
pub fn solve_part2(input: &[Hand]) -> usize {
    input
        .iter()
        .sorted()
        .rev()
        .enumerate()
        .map(|(i, hand)| (i + 1) * hand.bid)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::Card::*;
    use super::*;
    use std::vec;

    const EXAMPLE_INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn parse_example_input() {
        assert_eq!(
            parse_input_part1(EXAMPLE_INPUT)[0],
            Hand {
                cards: vec![Three, Two, Ten, Three, King],
                bid: 765,
            }
        )
    }

    #[test]
    fn solve_example_part1() {
        assert_eq!(solve_part1(&parse_input_part1(EXAMPLE_INPUT)), 6440)
    }

    #[test]
    fn solve_example_part2() {
        assert_eq!(solve_part2(&parse_input_part2(EXAMPLE_INPUT)), 5905)
    }

    #[test]
    fn test_hand_types() {
        assert_eq!(
            Hand::try_from("AAAAA 1").unwrap().get_type(),
            CardsType::FiveOKind
        );
        assert_eq!(
            Hand::try_from("T5555 1").unwrap().get_type(),
            CardsType::FourOKind
        );
        assert_eq!(
            Hand::try_from("T55T5 1").unwrap().get_type(),
            CardsType::FullHouse
        );
        assert_eq!(
            Hand::try_from("T55J5 1").unwrap().get_type(),
            CardsType::ThreeOKind
        );
        assert_eq!(Hand::try_from("KK677 1").unwrap().get_type(), CardsType::TwoPair);
        assert_eq!(Hand::try_from("32T3K 1").unwrap().get_type(), CardsType::OnePair);
        assert_eq!(
            Hand::try_from("32T4K 1").unwrap().get_type(),
            CardsType::HighCard
        );
    }

    #[test]
    fn test_hand_types_part2() {
        assert_eq!(
            Hand::try_from("*AAAA 1").unwrap().get_type(),
            CardsType::FiveOKind
        );

        assert_eq!(
            Hand::try_from("***** 1").unwrap().get_type(),
            CardsType::FiveOKind
        );

        assert_eq!(
            Hand::try_from("T5*** 1").unwrap().get_type(),
            CardsType::FourOKind
        );
        assert_eq!(
            Hand::try_from("T*5T5 1").unwrap().get_type(),
            CardsType::FullHouse
        );
        assert_eq!(
            Hand::try_from("T*5J5 1").unwrap().get_type(),
            CardsType::ThreeOKind
        );
        assert_eq!(
            Hand::try_from("K66*7 1").unwrap().get_type(),
            CardsType::ThreeOKind
        );
        assert_eq!(Hand::try_from("32T*K 1").unwrap().get_type(), CardsType::OnePair);
        assert_eq!(Hand::try_from("32T4* 1").unwrap().get_type(), CardsType::OnePair);
    }
}
