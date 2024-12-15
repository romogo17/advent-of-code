use glam::IVec2;
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

#[tracing::instrument(skip(input))]
pub fn process(input: &str, map_size: IVec2) -> miette::Result<u32> {
    let (_input, mut robots) = parse(input).map_err(|e| miette!("parse failed: {}", e))?;

    debug!("Starting map {}", robots_map_to_string(&robots, &map_size));
    for _i in 0..100 {
        for robot in robots.iter_mut() {
            robot.position = (robot.position + robot.velocity).rem_euclid(map_size);
        }
    }
    debug!("After 100s {}", robots_map_to_string(&robots, &map_size));

    let middles = map_size / 2;
    let quadrants = [
        (0..middles.x, 0..middles.y),
        ((middles.x + 1)..map_size.x, 0..middles.y),
        (0..middles.x, (middles.y + 1)..map_size.y),
        ((middles.x + 1)..map_size.x, (middles.y + 1)..map_size.y),
    ];

    let result: usize = quadrants
        .iter()
        .map(|(xs, ys)| {
            robots
                .iter()
                .filter(|Robot { position, .. }| {
                    xs.contains(&position.x) && ys.contains(&position.y)
                })
                .count()
        })
        .product();

    Ok(result as u32)
}

#[cfg(test)]
mod day_14_part1 {
    use super::*;

    #[test_log::test]
    fn example() -> miette::Result<()> {
        let input = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";
        assert_eq!(12, process(input, IVec2::new(11, 7))?);
        Ok(())
    }

    #[test_log::test]
    fn input() -> miette::Result<()> {
        let input = include_str!("../inputs/input.txt");
        assert_eq!(229069152, process(input, IVec2::new(101, 103))?);
        Ok(())
    }
}
