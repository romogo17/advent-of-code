use crate::custom_error::AocError;
use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending},
    multi::separated_list0,
    sequence::separated_pair,
    IResult,
};

fn parse(input: &str) -> IResult<&str, (Vec<u64>, Vec<u64>)> {
    let (input, output) = separated_list0(
        line_ending,
        separated_pair(complete::u64, tag("   "), complete::u64),
    )(input)?;

    Ok((input, output.into_iter().unzip()))
}

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<u64, AocError> {
    let (_, (mut x, mut y)) = parse(input).expect("should parse input");
    x.sort();
    y.sort();

    let total_distance: u64 = x
        .into_iter()
        .zip(y.into_iter())
        .map(|(l, r)| match l.cmp(&r) {
            std::cmp::Ordering::Less => r - l,
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Greater => l - r,
        })
        .sum();

    Ok(total_distance)
}

#[cfg(test)]
mod day_01_part1 {
    use super::*;

    #[test_log::test]
    fn example() -> miette::Result<()> {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        assert_eq!(11, process(input)?);
        Ok(())
    }

    #[test_log::test]
    fn input() -> miette::Result<()> {
        let input = include_str!("../inputs/input.txt");
        assert_eq!(1830467, process(input)?);
        Ok(())
    }
}
