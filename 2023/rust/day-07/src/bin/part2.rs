use nom::{
    character::complete::{self, alphanumeric1, space1},
    sequence::separated_pair,
    IResult,
};
use std::collections::HashMap;

#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq)]
enum CardValue {
    Joker, // wildcard
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Queen, // 12
    King,  // 13
    Ace,   // 14
}

impl CardValue {
    fn new(input: char) -> Self {
        match input {
            '2' => CardValue::Two,
            '3' => CardValue::Three,
            '4' => CardValue::Four,
            '5' => CardValue::Five,
            '6' => CardValue::Six,
            '7' => CardValue::Seven,
            '8' => CardValue::Eight,
            '9' => CardValue::Nine,
            'T' => CardValue::Ten,
            'J' => CardValue::Joker,
            'Q' => CardValue::Queen,
            'K' => CardValue::King,
            'A' => CardValue::Ace,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq)]
struct Cards(Vec<CardValue>);

impl Cards {
    fn new(input: &str) -> Self {
        Cards(input.chars().map(|c| CardValue::new(c)).collect())
    }
}

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq)]
enum Hand {
    HighCard(Cards),
    OnePair(Cards),
    TwoPairs(Cards),
    ThreeOfAKind(Cards),
    FullHouse(Cards),
    FourOfAKind(Cards),
    FiveOfAKind(Cards),
}

impl Hand {
    fn new(input: &str) -> Self {
        let mut map: HashMap<char, u32> = input.chars().fold(HashMap::new(), |mut acc, c| {
            *acc.entry(c).or_insert(0) += 1;
            acc
        });

        match map.remove(&'J') {
            Some(j_count) => {
                let mut counts: Vec<&u32> = map.values().collect();
                counts.sort();

                match (j_count, &counts[..]) {
                    (1, &[4]) => return Hand::FiveOfAKind(Cards::new(input)),
                    (1, &[1, 3]) => return Hand::FourOfAKind(Cards::new(input)),
                    (1, &[2, 2]) => return Hand::FullHouse(Cards::new(input)),
                    (1, &[1, 1, 2]) => return Hand::ThreeOfAKind(Cards::new(input)),
                    (1, &[1, 1, 1, 1]) => return Hand::OnePair(Cards::new(input)),

                    (2, &[3]) => return Hand::FiveOfAKind(Cards::new(input)),
                    (2, &[1, 2]) => return Hand::FourOfAKind(Cards::new(input)),
                    (2, &[1, 1, 1]) => return Hand::ThreeOfAKind(Cards::new(input)),

                    (3, &[2]) => return Hand::FiveOfAKind(Cards::new(input)),
                    (3, &[1, 1]) => return Hand::FourOfAKind(Cards::new(input)),

                    (4, &[1]) => return Hand::FiveOfAKind(Cards::new(input)),

                    (5, &[]) => return Hand::FiveOfAKind(Cards::new(input)),

                    _ => unreachable!(),
                }
            }
            None => {
                let mut counts: Vec<&u32> = map.values().collect();
                counts.sort();

                match &counts[..] {
                    &[5] => return Hand::FiveOfAKind(Cards::new(input)),
                    &[1, 4] => return Hand::FourOfAKind(Cards::new(input)),
                    &[2, 3] => return Hand::FullHouse(Cards::new(input)),
                    &[1, 1, 3] => return Hand::ThreeOfAKind(Cards::new(input)),
                    &[1, 2, 2] => return Hand::TwoPairs(Cards::new(input)),
                    &[1, 1, 1, 2] => return Hand::OnePair(Cards::new(input)),
                    &[1, 1, 1, 1, 1] => return Hand::HighCard(Cards::new(input)),
                    _ => unreachable!(),
                }
            }
        }
    }
}

fn main() {
    let input = include_str!("../../inputs/input1.txt");
    let output = process(input);
    println!("Output is {output}");
}

fn process(input: &str) -> u64 {
    let mut hands: Vec<(Hand, u64)> = input
        .lines()
        .map(|line| {
            let (_, (hand, score)) = parse_line(line).unwrap();
            (Hand::new(hand), score)
        })
        .collect();

    hands.sort_by(|(hand_a, _), (hand_b, _)| hand_a.cmp(hand_b));
    hands
        .iter()
        .enumerate()
        .map(|(idx, (_, bid))| (idx as u64 + 1) * bid)
        .sum()
}

fn parse_line(input: &str) -> IResult<&str, (&str, u64)> {
    separated_pair(alphanumeric1, space1, complete::u64)(input)
}

#[cfg(test)]
mod day_07_part1 {
    use super::*;

    #[test]
    fn example() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        let output = process(input);
        assert_eq!(output, 5905);
    }

    #[test]
    fn input1() {
        let input = include_str!("../../inputs/input1.txt");
        let output = process(input);
        assert_eq!(output, 246894760);
    }

    #[test]
    fn diff_hand_type_sort() {
        assert!(Hand::new("AA8AA") < Hand::new("AAAAA"));
        assert!(Hand::new("23332") < Hand::new("AA8AA"));
        assert!(Hand::new("TTT98") < Hand::new("23332"));
        assert!(Hand::new("23432") < Hand::new("TTT98"));
        assert!(Hand::new("A23A4") < Hand::new("23432"));
        assert!(Hand::new("23456") < Hand::new("A23A4"));
    }

    #[test]
    fn same_hand_type_sort() {
        assert!(Hand::new("2AAAA") < Hand::new("33332"));
        assert!(Hand::new("77788") < Hand::new("77888"));
    }

    #[test]
    fn joker_sort() {
        assert!(Hand::new("T55J5") < Hand::new("QQQJA"));
        assert!(Hand::new("QQQJA") < Hand::new("KTJJT"));
    }
}
