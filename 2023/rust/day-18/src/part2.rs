use crate::custom_error::AocError;
use tracing::{debug, info};

use glam::I64Vec2;
use itertools::Itertools;
use nom::{
    bytes::complete::{take, take_until},
    character::complete::{self},
    multi::many1,
    sequence::terminated,
    IResult,
};

#[derive(Debug)]
struct DigInstruction {
    direction: I64Vec2,
    count: i64,
}

fn dig_instruction(input: &str) -> IResult<&str, DigInstruction> {
    let (input, _) = terminated(take_until("#"), complete::char('#'))(input)?;
    let (input, hex) = take(5usize)(input)?;
    let (input, direction) = take(1usize)(input)?;

    let count = i64::from_str_radix(hex, 16).expect("a valid hex number");
    let direction = match i64::from_str_radix(direction, 16).expect("a valid hex number") {
        0 => I64Vec2::X,
        1 => I64Vec2::Y,
        2 => I64Vec2::NEG_X,
        3 => I64Vec2::NEG_Y,
        _ => panic!("invalid direction"),
    };

    Ok((input, DigInstruction { direction, count }))
}

fn parse_dig_plan(input: &str) -> IResult<&str, Vec<DigInstruction>> {
    many1(dig_instruction)(input)
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
mod day_18_part2 {
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
        assert_eq!(952408144115, process(input)?);
        Ok(())
    }

    #[test_log::test]
    fn input1() -> miette::Result<()> {
        let input = include_str!("../inputs/input1.txt");
        assert_eq!(72811019847283, process(input)?);
        Ok(())
    }
}
