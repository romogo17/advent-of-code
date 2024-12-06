use std::{cmp::Ordering, collections::HashMap};

use crate::custom_error::AocError;
use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending},
    multi::{fold_many1, separated_list1},
    sequence::{separated_pair, terminated},
    IResult,
};
use tracing::{debug, info};

fn rules(input: &str) -> IResult<&str, HashMap<u32, Vec<u32>>> {
    fold_many1(
        terminated(
            separated_pair(complete::u32, tag("|"), complete::u32),
            line_ending,
        ),
        HashMap::default,
        |mut acc: HashMap<u32, Vec<u32>>, (page, after)| {
            acc.entry(page)
                .and_modify(|afters| afters.push(after))
                .or_insert(vec![after]);
            acc
        },
    )(input)
}

fn updates(input: &str) -> IResult<&str, Vec<Vec<u32>>> {
    separated_list1(line_ending, separated_list1(tag(","), complete::u32))(input)
}

fn parse(input: &str) -> IResult<&str, (HashMap<u32, Vec<u32>>, Vec<Vec<u32>>)> {
    let (input, rules) = terminated(rules, line_ending)(input)?;
    let (input, updates) = updates(input)?;

    Ok((input, (rules, updates)))
}

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<u32, AocError> {
    let (_input, (rules, updates)) = parse(input).expect("should parse input");
    debug!(?rules);
    debug!(?updates);

    let results: Vec<usize> = updates
        .iter()
        .enumerate()
        .filter_map(|(idx, update)| {
            info!(?update, "validating");

            let mut current_page = update[0];
            let mut remaining_pages = &update[1..];
            let mut processed_pages = &update[0..0];

            while processed_pages.len() != update.len() {
                debug!(?current_page, ?processed_pages, ?remaining_pages);

                // check if any of the pages that was already processed (and hence is before) must actually go after the current page
                if let Some(pages_that_must_come_after) = rules.get(&current_page) {
                    if !pages_that_must_come_after
                        .iter()
                        .all(|page| !processed_pages.contains(page))
                    {
                        return Some(idx);
                    }
                }

                // next iteration
                processed_pages = &update[0..(processed_pages.len() + 1)];
                if let Some(page) = remaining_pages.get(0) {
                    current_page = *page;
                    remaining_pages = &remaining_pages[1..];
                }
            }

            None
        })
        .collect();

    let sorted_updates: Vec<_> = results
        .iter()
        .map(|idx| {
            let mut update = updates[*idx].clone();
            update.sort_by(|a, b| {
                if rules.get(a).is_some_and(|pages| pages.contains(b)) {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            });
            update
        })
        .collect();

    let result: u32 = sorted_updates
        .iter()
        .map(|update| {
            let middle_idx = update.len() / 2;
            update[middle_idx]
        })
        .sum();

    Ok(result)
}

#[cfg(test)]
mod day_05_part2 {
    use super::*;

    #[test_log::test]
    fn example() -> miette::Result<()> {
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
        assert_eq!(123, process(input)?);
        Ok(())
    }

    #[test_log::test]
    fn input() -> miette::Result<()> {
        let input = include_str!("../inputs/input.txt");
        assert_eq!(5900, process(input)?);
        Ok(())
    }
}
