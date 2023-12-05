use day_04::*;
use std::collections::BTreeMap;

fn main() {
    let input = include_str!("../../inputs/input1.txt");
    let output = process(input);
    println!("Output is {output}");
}

fn process(input: &str) -> u32 {
    let cards = parse_cards(input);

    let results_map = cards
        .iter()
        .map(|card| {
            let intersection = card.winning_numbers.intersection(&card.my_numbers);
            let copies_won = intersection.count();

            (
                card.id,
                match copies_won {
                    0 => vec![],
                    _ => ((card.id + 1)..(card.id + copies_won as u32 + 1)).collect::<Vec<u32>>(),
                },
            )
        })
        .collect::<BTreeMap<u32, Vec<u32>>>();

    results_map
        .keys()
        .map(|key| 1 + won_copies_of(*key, &results_map))
        .sum()
}

fn won_copies_of(card_id: u32, results_map: &BTreeMap<u32, Vec<u32>>) -> u32 {
    // shim function for the memoization logic
    fn won_copies_of_memo(
        card_id: u32,
        results_map: &BTreeMap<u32, Vec<u32>>,
        memo: &mut BTreeMap<u32, u32>,
        rec_level: usize,
    ) -> u32 {
        match memo.get(&card_id).map(|entry| entry.clone()) {
            Some(result) => result,
            None => {
                let result = match results_map.get(&card_id) {
                    Some(copies) if copies.len() == 0 => 0,
                    Some(copies) => copies
                        .iter()
                        .map(|copy| 1 + won_copies_of_memo(*copy, results_map, memo, rec_level + 1))
                        .sum(),
                    None => unimplemented!("Card {} not found in results", card_id),
                };
                memo.insert(card_id, result.clone());
                result
            }
        }
    }

    won_copies_of_memo(card_id, results_map, &mut BTreeMap::new(), 1)
}

#[cfg(test)]
mod day_04_part2 {
    use super::*;

    #[test]
    fn example() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let output = process(input);
        assert_eq!(output, 30);
    }

    #[test]
    fn input1() {
        let input = include_str!("../../inputs/input1.txt");
        let output = process(input);
        assert_eq!(output, 8736438);
    }
}
