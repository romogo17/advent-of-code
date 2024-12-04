use nom::{
    bytes::complete::tag,
    character::complete::{self, anychar},
    multi::{many1, many_till},
    sequence::{delimited, separated_pair},
    IResult, Parser,
};

use crate::custom_error::AocError;
use tracing::debug;

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
enum Instruction {
    Mul(u64, u64),
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("mul")(input)?;

    let (input, pair) = delimited(
        tag("("),
        separated_pair(complete::u64, tag(","), complete::u64),
        tag(")"),
    )(input)?;

    Ok((input, Instruction::Mul(pair.0, pair.1)))
}

fn parse(input: &str) -> IResult<&str, Vec<Instruction>> {
    many1(many_till(anychar, parse_instruction).map(|(_discard, instruction)| instruction))(input)
}

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<u64, AocError> {
    let (_, instructions) = parse(input).expect("should parse input");
    debug!(?instructions);

    let result: u64 = instructions
        .iter()
        .map(|ins| match ins {
            Instruction::Mul(a, b) => a * b,
        })
        .sum();

    Ok(result)
}

#[cfg(test)]
mod day_03_part1 {
    use super::*;

    #[test_log::test]
    fn example() -> miette::Result<()> {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(161, process(input)?);
        Ok(())
    }

    #[test_log::test]
    fn input() -> miette::Result<()> {
        let input = include_str!("../inputs/input.txt");
        assert_eq!(173785482, process(input)?);
        Ok(())
    }
}
