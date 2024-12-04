use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, anychar},
    combinator::value,
    multi::{many1, many_till},
    sequence::{delimited, separated_pair},
    IResult, Parser,
};

use crate::custom_error::AocError;
use tracing::debug;

#[derive(Debug, Clone)]
enum Instruction {
    Mul(u64, u64),
    Do,
    Dont,
}

#[derive(PartialEq, Eq)]
enum Toggle {
    On,
    Off,
}

fn mul(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("mul")(input)?;

    let (input, pair) = delimited(
        tag("("),
        separated_pair(complete::u64, tag(","), complete::u64),
        tag(")"),
    )(input)?;

    Ok((input, Instruction::Mul(pair.0, pair.1)))
}

fn instruction(input: &str) -> IResult<&str, Instruction> {
    alt((
        value(Instruction::Dont, tag("don't()")),
        value(Instruction::Do, tag("do()")),
        mul,
    ))(input)
}

fn parse(input: &str) -> IResult<&str, Vec<Instruction>> {
    many1(many_till(anychar, instruction).map(|(_discard, instruction)| instruction))(input)
}

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<u64, AocError> {
    let (_, instructions) = parse(input).expect("should parse input");
    debug!(?instructions);

    let (_, result) = instructions
        .iter()
        .fold((Toggle::On, 0), |(state, acc), ins| match ins {
            Instruction::Mul(a, b) => {
                if state == Toggle::On {
                    (state, acc + a * b)
                } else {
                    (state, acc)
                }
            }
            Instruction::Do => (Toggle::On, acc),
            Instruction::Dont => (Toggle::Off, acc),
        });

    Ok(result)
}
#[cfg(test)]
mod day_03_part2 {
    use super::*;

    #[test_log::test]
    fn example() -> miette::Result<()> {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(48, process(input)?);
        Ok(())
    }

    #[test_log::test]
    fn input() -> miette::Result<()> {
        let input = include_str!("../inputs/input.txt");
        assert_eq!(83158140, process(input)?);
        Ok(())
    }
}
