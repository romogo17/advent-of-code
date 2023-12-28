use crate::custom_error::AocError;
use ndarray::prelude::*;
use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending, space1},
    multi::separated_list1,
    sequence::{delimited, separated_pair, terminated},
    IResult, Parser,
};
// use num::FromPrimitive;
use tracing::debug;

// type CordType = num::rational::Ratio<i128>;

#[derive(Debug)]
struct DHailstone {
    starting_position: Array1<f64>,
    direction: Array1<f64>,
}

fn array_xyz(input: &str) -> IResult<&str, Array1<f64>> {
    let (input, a) = complete::i64(input)?;
    let (input, _) = terminated(tag(","), space1)(input)?;
    let (input, b) = complete::i64(input)?;
    let (input, _) = terminated(tag(","), space1)(input)?;
    let (input, c) = complete::i64(input)?;

    Ok((
        input,
        array![
            a as f64,
            b as f64,
            c as f64,
        ],
    ))
}

fn parse_hailstones(input: &str) -> IResult<&str, Vec<DHailstone>> {
    let (input, hailstones) = separated_list1(
        line_ending,
        separated_pair(array_xyz, delimited(space1, tag("@"), space1), array_xyz).map(
            |(starting_position, direction)| DHailstone {
                starting_position,
                direction,
            },
        ),
    )(input)?;

    Ok((input, hailstones))
}

fn cross(a: &Array1<f64>, b: &Array1<f64>) -> Array1<f64> {
    array![
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0]
    ]
}

#[rustfmt::skip]
fn cross_matrix(v: &Array1<f64>) -> Array2<f64> {
    array![
        [0., -v[2], v[1]], 
        [v[2], 0., -v[0]], 
        [-v[1], v[0], 0.]
    ]
}

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<u64, AocError> {
    let (_, hailstones) = parse_hailstones(input).expect("should parse hailstones");

    debug!("{:?}", hailstones[0]);
    debug!("{:?}", hailstones[1]);
    debug!("{:?}", hailstones[2]);

    let mut a: Array2<f64> = Array::zeros((6, 6));
    let mut b: Array1<f64> = Array::zeros(6);

    b.slice_mut(s![0..3]).assign(
        &(-cross(&hailstones[0].starting_position, &hailstones[0].direction)
            + cross(&hailstones[1].starting_position, &hailstones[1].direction)),
    );
    b.slice_mut(s![3..6]).assign(
        &(-cross(&hailstones[0].starting_position, &hailstones[0].direction)
            + cross(&hailstones[2].starting_position, &hailstones[2].direction)),
    );
    debug!("{:?}", b);

    a.slice_mut(s![0..3, 0..3])
        .assign(&(cross_matrix(&hailstones[0].direction) - cross_matrix(&hailstones[1].direction)));
    a.slice_mut(s![3..6, 0..3])
        .assign(&(cross_matrix(&hailstones[0].direction) - cross_matrix(&hailstones[2].direction)));
    a.slice_mut(s![0..3, 3..6]).assign(
        &(-cross_matrix(&hailstones[0].starting_position)
            + cross_matrix(&hailstones[1].starting_position)),
    );
    a.slice_mut(s![3..6, 3..6]).assign(
        &(-cross_matrix(&hailstones[0].starting_position)
            + cross_matrix(&hailstones[2].starting_position)),
    );
    debug!("{:?}", a);

    let a_inv = a.inv().unwrap();
    let result = a_inv.dot(&b);
    debug!("{:?}", result);

    Ok(result.slice(s![0..3]).map(|x| x.round() as u64).sum())
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
