use crate::custom_error::AocError;
use tracing::{debug, info};

use glam::I64Vec2;
use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, hex_digit1, line_ending, space1},
    multi::separated_list1,
    sequence::delimited,
    IResult, Parser,
};

#[derive(Debug)]
struct DigInstruction<'a> {
    direction: I64Vec2,
    count: i64,
    #[allow(dead_code)]
    hex_color: &'a str,
}

fn dig_instruction(input: &str) -> IResult<&str, DigInstruction> {
    let (input, direction) = alt((
        complete::char('R').map(|_| I64Vec2::X),
        complete::char('L').map(|_| I64Vec2::NEG_X),
        complete::char('U').map(|_| I64Vec2::NEG_Y),
        complete::char('D').map(|_| I64Vec2::Y),
    ))(input)?;

    let (input, count) = delimited(space1, complete::i64, space1)(input)?;

    let (input, hex_color) = delimited(tag("(#"), hex_digit1, complete::char(')'))(input)?;

    Ok((
        input,
        DigInstruction {
            direction,
            count,
            hex_color,
        },
    ))
}

fn parse_dig_plan(input: &str) -> IResult<&str, Vec<DigInstruction>> {
    separated_list1(line_ending, dig_instruction)(input)
}

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<u64, AocError> {
    let (_, dig_plan) = parse_dig_plan(input).expect("a valid dig plan");

    let vertices = dig_plan
        .iter()
        .scan(I64Vec2::ZERO, |state, instruction| {
            *state += instruction.direction * instruction.count;
            Some(*state)
        })
        .collect_vec();

    let perimeter_len = vertices
        .iter()
        .tuple_windows()
        .map(|(a, b)| {
            let distance = (*a - *b).abs();
            distance.x + distance.y
        })
        .sum::<i64>()
        + {
            // wraparound from the last vertex to the first
            let last = vertices.last().unwrap();
            let first = vertices.first().unwrap();
            let distance = (*first - *last).abs();
            distance.x + distance.y
        };
    debug!(?perimeter_len);

    let area = ((vertices
        .iter()
        .tuple_windows()
        .map(|(a, b)| a.x * b.y - a.y * b.x)
        .sum::<i64>()
        + perimeter_len)
        / 2)
    .abs()
        + 1;

    info!(?area);

    Ok(area as u64)
}

#[cfg(test)]
mod day_18_part1 {
    use super::*;

    #[test_log::test]
    fn example() -> miette::Result<()> {
        let input = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";
        assert_eq!(62, process(input)?);
        Ok(())
    }

    #[test_log::test]
    fn input1() -> miette::Result<()> {
        let input = include_str!("../inputs/input1.txt");
        assert_eq!(48400, process(input)?);
        Ok(())
    }
}
