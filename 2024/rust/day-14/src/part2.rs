use glam::IVec2;
use itertools::Itertools;
use miette::miette;
use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending, space1},
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult, Parser,
};
use tracing::debug;

#[derive(Debug)]
struct Robot {
    position: IVec2,
    velocity: IVec2,
}

fn parse_ivec2(input: &str) -> IResult<&str, IVec2> {
    let (input, (x, y)) = separated_pair(complete::i32, tag(","), complete::i32)(input)?;
    Ok((input, IVec2::new(x, y)))
}

fn parse(input: &str) -> IResult<&str, Vec<Robot>> {
    separated_list1(
        line_ending,
        separated_pair(
            preceded(tag("p="), parse_ivec2),
            space1,
            preceded(tag("v="), parse_ivec2),
        )
        .map(|(position, velocity)| Robot { position, velocity }),
    )(input)
}

fn robots_map_to_string(robots: &[Robot], map_size: &IVec2) -> String {
    let mut str: String = String::from("\n");
    for y in 0..map_size.y {
        for x in 0..map_size.x {
            match robots
                .iter()
                .filter(|Robot { position, .. }| position.x == x && position.y == y)
                .count()
            {
                0 => str += ".",
                n => str += &format!("{}", n),
            }
        }
        str += "\n"
    }
    str
}

fn tree_test(robots: &[Robot]) -> bool {
    robots
        .iter()
        .map(|Robot { position, .. }| position)
        .all_unique()
}

#[tracing::instrument(skip(input))]
pub fn process(input: &str, map_size: IVec2) -> miette::Result<u32> {
    let (_input, mut robots) = parse(input).map_err(|e| miette!("parse failed: {}", e))?;

    debug!("Starting map {}", robots_map_to_string(&robots, &map_size));

    let mut i = 0;
    let result = loop {
        for robot in robots.iter_mut() {
            robot.position = (robot.position + robot.velocity).rem_euclid(map_size);
        }
        i += 1;
        if tree_test(&robots) {
            break i;
        }
    };
    debug!("After {}s {}", i, robots_map_to_string(&robots, &map_size));

    Ok(result as u32)
}

#[cfg(test)]
mod day_14_part2 {
    use super::*;

    #[test_log::test]
    fn input() -> miette::Result<()> {
        let input = include_str!("../inputs/input.txt");
        assert_eq!(7383, process(input, IVec2::new(101, 103))?);
        Ok(())
    }
}
