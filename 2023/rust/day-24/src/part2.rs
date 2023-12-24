use crate::custom_error::AocError;
use glam::{DVec3, I64Vec3};
use ndarray_linalg::error::LinalgError;
use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending, space1},
    multi::separated_list1,
    sequence::{delimited, separated_pair, terminated},
    IResult, Parser,
};
use tracing::debug;

#[derive(Debug)]
struct DHailstone {
    starting_position: DVec3,
    direction: DVec3,
}

impl DHailstone {
    fn solve_intersection_xyx(
        &self,
        second: &DHailstone,
        third: &DHailstone,
    ) -> Result<DHailstone, LinalgError> {
        use ndarray::prelude::*;
        use ndarray_linalg::Solve;

        let v1 = vec![self.direction.x, self.direction.y, self.direction.z];
        let p1 = vec![
            self.starting_position.x,
            self.starting_position.y,
            self.starting_position.z,
        ];

        let v2 = vec![second.direction.x, second.direction.y, second.direction.z];
        let p2 = vec![
            second.starting_position.x,
            second.starting_position.y,
            second.starting_position.z,
        ];

        let v3 = vec![third.direction.x, third.direction.y, third.direction.z];
        let p3 = vec![
            third.starting_position.x,
            third.starting_position.y,
            third.starting_position.z,
        ];

        let a: Array2<f64> = array![
            [
                -(v1[1] - v2[1]),
                v1[0] - v2[0],
                0.,
                p1[1] - p2[1],
                -(p1[0] - p2[0]),
                0.
            ],
            [
                -(v1[1] - v3[1]),
                v1[0] - v3[0],
                0.,
                p1[1] - p3[1],
                -(p1[0] - p3[0]),
                0.
            ],
            [
                0.,
                -(v1[2] - v2[2]),
                v1[1] - v2[1],
                0.,
                p1[2] - p2[2],
                -(p1[1] - p2[1])
            ],
            [
                0.,
                -(v1[2] - v3[2]),
                v1[1] - v3[1],
                0.,
                p1[2] - p3[2],
                -(p1[1] - p3[1])
            ],
            [
                -(v1[2] - v2[2]),
                0.,
                v1[0] - v2[0],
                p1[2] - p2[2],
                0.,
                -(p1[0] - p2[0])
            ],
            [
                -(v1[2] - v3[2]),
                0.,
                v1[0] - v3[0],
                p1[2] - p3[2],
                0.,
                -(p1[0] - p3[0])
            ]
        ];
        let b: Array1<f64> = array![
            (p1[1] * v1[0] - p2[1] * v2[0]) - (p1[0] * v1[1] - p2[0] * v2[1]),
            (p1[1] * v1[0] - p3[1] * v3[0]) - (p1[0] * v1[1] - p3[0] * v3[1]),
            (p1[2] * v1[1] - p2[2] * v2[1]) - (p1[1] * v1[2] - p2[1] * v2[2]),
            (p1[2] * v1[1] - p3[2] * v3[1]) - (p1[1] * v1[2] - p3[1] * v3[2]),
            (p1[2] * v1[0] - p2[2] * v2[0]) - (p1[0] * v1[2] - p2[0] * v2[2]),
            (p1[2] * v1[0] - p3[2] * v3[0]) - (p1[0] * v1[2] - p3[0] * v3[2])
        ];
        let coefficients = a.solve_into(b).unwrap();

        Ok(DHailstone {
            starting_position: DVec3::from([coefficients[0], coefficients[1], coefficients[2]]),
            direction: DVec3::from([coefficients[3], coefficients[4], coefficients[5]]),
        })
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

fn parse_hailstones(input: &str) -> IResult<&str, Vec<DHailstone>> {
    let (input, hailstones) = separated_list1(
        line_ending,
        separated_pair(i64vec3, delimited(space1, tag("@"), space1), i64vec3).map(
            |(starting_position, direction)| DHailstone {
                starting_position: starting_position.as_dvec3(),
                direction: direction.as_dvec3(),
            },
        ),
    )(input)?;

    Ok((input, hailstones))
}

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<u64, AocError> {
    let (_, hailstones) = parse_hailstones(input).expect("should parse hailstones");

    let intersection = hailstones[0]
        .solve_intersection_xyx(&hailstones[1], &hailstones[2])
        .expect("should have an intersection");
    debug!(?intersection);

    let sum = intersection.starting_position.x.round()
        + intersection.starting_position.y.round()
        + intersection.starting_position.z.round();
    debug!("Sum of xyz {}", sum);

    Ok(sum as u64)
}
#[cfg(test)]
mod day_24_part2 {
    use super::*;

    #[test_log::test]
    fn example() -> miette::Result<()> {
        let input = "19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3";
        assert_eq!(47, process(input)?);
        Ok(())
    }
}
