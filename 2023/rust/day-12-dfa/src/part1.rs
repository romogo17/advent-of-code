use std::collections::HashMap;

use crate::custom_error::AocError;
use nom::{
    bytes::complete::{is_a, tag},
    character::complete::{self, multispace0, space1},
    combinator::all_consuming,
    multi::{many1, separated_list1},
    sequence::{separated_pair, terminated},
    IResult, Parser,
};
use tracing::debug;

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
struct State {
    label: String,
    dot: Option<usize>,
    hash: Option<usize>,
}

// Implementation based on
// https://alexoxorn.github.io/posts/aoc-day12-regular_languages/
// https://github.com/AlexOxorn/AdventOfCode/blob/main/puzzles/2023/src/day12.cpp
#[derive(Debug, Clone, PartialEq, Eq)]
struct DFA {
    states: Vec<State>,
}

impl DFA {
    fn new(damaged_pattern: &Vec<u64>) -> Self {
        let states_len =
            damaged_pattern.iter().map(|n| *n as usize).sum::<usize>() + damaged_pattern.len();
        let mut states = vec![State::default(); states_len];

        states[0].label = node_label(0);
        states[0].dot = Some(0);
        states[0].hash = Some(1);

        let mut i = 1;
        for b in damaged_pattern.iter() {
            for _ in 0..*b - 1 {
                states[i].label = node_label(i);
                states[i].hash = Some(i + 1);

                i += 1;
            }

            if i + 2 < states.len() {
                states[i].label = node_label(i);
                states[i].dot = Some(i + 1);

                i += 1;

                states[i].label = node_label(i);
                states[i].dot = Some(i);
                states[i].hash = Some(i + 1);
            }
            i += 1;
        }

        states[states_len - 1].label = node_label(states_len - 1);
        states[states_len - 1].dot = Some(states_len - 1);

        DFA { states }
    }

    fn count(&self, pattern: &String) -> usize {
        let mut curr = HashMap::new();
        curr.insert(&self.states[0], 1usize);

        for c in pattern.chars() {
            let mut next = HashMap::with_capacity(self.states.len());
            for (key, value) in curr.iter() {
                if (c == '.' || c == '?') && key.dot.is_some() {
                    next.entry(&self.states[key.dot.unwrap()])
                        .and_modify(|e| *e += value)
                        .or_insert(*value);
                }
                if (c == '#' || c == '?') && key.hash.is_some() {
                    next.entry(&self.states[key.hash.unwrap()])
                        .and_modify(|e| *e += value)
                        .or_insert(*value);
                }
            }
            curr = next;
        }

        *curr.get(&self.states[self.states.len() - 1]).unwrap()
    }
}

fn node_label(mut num: usize) -> String {
    num += 1;
    let mut label = String::new();
    while num > 0 {
        num -= 1;
        let character = (b'A' + (num % 26) as u8) as char;
        label.insert(0, character);
        num /= 26;
    }
    label
}

fn parse_records(input: &str) -> IResult<&str, Vec<(String, Vec<u64>)>> {
    let (input, output) = all_consuming(many1(terminated(
        separated_pair(
            is_a("#.?").map(|line| String::from(line)),
            space1,
            separated_list1(tag(","), complete::u64),
        ),
        multispace0,
    )))(input)?;

    Ok((input, output))
}

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<u64, AocError> {
    let (_, records) = parse_records(input).expect("should parse the records");

    let result = records
        .iter()
        .map(|record| {
            let (pattern, damaged_pattern) = record;
            let dfa = DFA::new(damaged_pattern);
            debug!(?pattern, ?damaged_pattern, ?dfa);
            let count = dfa.count(pattern);
            count as u64
        })
        .sum::<u64>();

    Ok(result)
}

#[cfg(test)]
mod day_12_dfa_part1 {
    use super::*;

    #[test_log::test]
    fn example() -> miette::Result<()> {
        let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
        assert_eq!(21, process(input)?);
        Ok(())
    }

    #[test_log::test]
    fn input1() -> miette::Result<()> {
        let input = include_str!("../inputs/input1.txt");
        assert_eq!(7191, process(input)?);
        Ok(())
    }
}
