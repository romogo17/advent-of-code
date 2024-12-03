use std::collections::HashMap;

use crate::custom_error::AocError;
use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending},
    multi::separated_list0,
    sequence::separated_pair,
    IResult,
};
use tracing::debug;

fn parse(input: &str) -> IResult<&str, (Vec<u64>, Vec<u64>)> {
    let (input, output) = separated_list0(
        line_ending,
        separated_pair(complete::u64, tag("   "), complete::u64),
    )(input)?;

    Ok((input, output.into_iter().unzip()))
}

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<u64, AocError> {
    let (_, (x, y)) = parse(input).expect("should parse input");
    let counts = y.iter().fold(HashMap::new(), |mut map, val| {
        map.entry(val).and_modify(|v| *v += 1).or_insert(1);
        map
    });
    debug!("{:?}", counts);

    let score = x.iter().fold(0, |acc, val| {
        acc + match counts.get(val) {
            Some(count) => *val * *count,
            _ => 0,
        }
    });

    Ok(score)
}

#[cfg(test)]
mod day_01_part2 {
    use super::*;

    #[test_log::test]
    fn example() -> miette::Result<()> {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        assert_eq!(31, process(input)?);
        Ok(())
    }

    #[test_log::test]
    fn input() -> miette::Result<()> {
        let input = include_str!("../inputs/input.txt");
        assert_eq!(26674158, process(input)?);
        Ok(())
    }
}
