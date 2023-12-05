use std::collections::HashSet;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Card {
    pub id: u32,
    pub winning_numbers: HashSet<u32>,
    pub my_numbers: HashSet<u32>,
}

pub fn parse_cards(input: &str) -> Vec<Card> {
    input.lines().map(|line| parse_card(line)).collect()
}

pub fn parse_card(input: &str) -> Card {
    let mut parts = input.split(": ");
    let id = parts
        .next()
        .unwrap()
        .trim()
        .split(" ")
        .filter(|id_str| !id_str.is_empty())
        .nth(1)
        .unwrap()
        .parse::<u32>()
        .unwrap();

    let numbers = parts
        .next()
        .unwrap()
        .trim()
        .split(" | ")
        .collect::<Vec<&str>>();

    Card {
        id,
        winning_numbers: numbers[0]
            .split(" ")
            .filter(|number_str| !number_str.is_empty())
            .map(|number_str| number_str.trim().parse::<u32>().unwrap())
            .collect(),
        my_numbers: numbers[1]
            .split(" ")
            .filter(|number_str| !number_str.is_empty())
            .map(|number_str| number_str.trim().parse::<u32>().unwrap())
            .collect(),
    }
}
