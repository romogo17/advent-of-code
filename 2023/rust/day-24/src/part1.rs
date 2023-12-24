use std::ops::RangeInclusive;

use crate::custom_error::AocError;
use glam::{DVec2, I64Vec3, Vec3Swizzles};
use itertools::Itertools;
use ndarray_linalg::error::LinalgError;
use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending, space1},
    multi::separated_list1,
    sequence::{delimited, separated_pair, terminated},
    IResult, Parser,
};
use tracing::debug;

#[derive(Debug, Eq, PartialEq, Clone)]
struct Hailstone {
    starting_position: I64Vec3,
    direction: I64Vec3,
}

impl Hailstone {
    fn at_xy(&self, scalar: f64) -> DVec2 {
        self.starting_position.as_dvec3().xy() + scalar * self.direction.as_dvec3().xy()
    }

    // https://math.stackexchange.com/questions/406864/intersection-of-two-lines-in-vector-form
    fn solve_intersection_xy(&self, other: &Hailstone) -> Result<(f64, f64, DVec2), LinalgError> {
        use ndarray::prelude::*;
        use ndarray_linalg::Solve;
        let a: Array2<f64> = array![
            [self.direction.x as f64, -other.direction.x as f64],
            [self.direction.y as f64, -other.direction.y as f64],
        ];

        let b: Array1<f64> = array![
            (other.starting_position.x - self.starting_position.x) as f64,
            (other.starting_position.y - self.starting_position.y) as f64,
        ];

        let x = a.solve_into(b)?;
        let self_scalar = x[0];
        let other_scalar = x[1];

        Ok((self_scalar, other_scalar, self.at_xy(self_scalar)))
    }
}

fn i64vec3(input: &str) -> IResult<&str, I64Vec3> {
    let (input, a) = complete::i64(input)?;
    let (input, _) = terminated(tag(","), space1)(input)?;
    let (input, b) = complete::i64(input)?;
    let (input, _) = terminated(tag(","), space1)(input)?;
    let (input, c) = complete::i64(input)?;

    Ok((input, I64Vec3::from([a, b, c])))
}

fn parse_hailstones(input: &str) -> IResult<&str, Vec<Hailstone>> {
    let (input, hailstones) = separated_list1(
        line_ending,
        separated_pair(i64vec3, delimited(space1, tag("@"), space1), i64vec3).map(
            |(starting_position, direction)| Hailstone {
                starting_position,
                direction,
            },
        ),
    )(input)?;

    Ok((input, hailstones))
}

#[tracing::instrument(skip(input, bounds))]
pub fn process(input: &str, bounds: RangeInclusive<f64>) -> miette::Result<u64, AocError> {
    let (_, hailstones) = parse_hailstones(input).expect("should parse hailstones");
    debug!(?hailstones);

    let results = hailstones
        .iter()
        .tuple_combinations()
        .filter_map(|(hail_one, hail_two)| {
            hail_one
                .solve_intersection_xy(&hail_two)
                .ok()
                .map(|intersection| ((hail_one, hail_two), intersection))
        })
        .filter(|(_hails, (hail_one_scalar, hail_two_scalar, coord))| {
            bounds.contains(&coord.x)
                && bounds.contains(&coord.y)
                && hail_one_scalar >= &0.0
                && hail_two_scalar >= &0.0
        })
        .collect::<Vec<((&Hailstone, &Hailstone), (f64, f64, DVec2))>>();

    for ((hail_one, hail_two), (scalar_one, scalar_two, coord)) in results.iter() {
        debug!(
            "Intersection found at {} between \n{:?} with scalar {}\n{:?} with scalar {}",
            coord, hail_one, scalar_one, hail_two, scalar_two,
        );
    }

    Ok(results.len() as u64)
}

#[cfg(test)]
mod day_24_part1 {
    use super::*;

    #[test_log::test]
    fn example() -> miette::Result<()> {
        let input = "19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3";
        let bounds = 7f64..=27f64;
        assert_eq!(2, process(input, bounds)?);
        Ok(())
    }

    #[test_log::test]
    fn input1() -> miette::Result<()> {
        let input = include_str!("../inputs/input1.txt");
        let bounds = 200000000000000f64..=400000000000000f64;
        assert_eq!(16050, process(input, bounds)?);
        Ok(())
    }
}
