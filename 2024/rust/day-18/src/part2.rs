use std::{
    fmt::{self, Write},
    ops::Not,
};

use glam::IVec2;
use miette::miette;
use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending},
    combinator::opt,
    multi::separated_list1,
    sequence::separated_pair,
    IResult, Parser,
};
use pathfinding::prelude::*;

const DIRECTIONS: [IVec2; 4] = [IVec2::X, IVec2::Y, IVec2::NEG_X, IVec2::NEG_Y];

fn parse(input: &str) -> IResult<&str, Vec<IVec2>> {
    let (input, bytes) = separated_list1(
        line_ending,
        separated_pair(complete::i32, tag(","), complete::i32).map(|(x, y)| IVec2::new(x, y)),
    )(input)?;
    let (input, _) = opt(line_ending)(input)?;

    Ok((input, bytes))
}

#[allow(dead_code)]
fn grid_to_string(tiles: &[IVec2], grid_size: IVec2) -> Result<String, fmt::Error> {
    let map_size = grid_size;

    let mut output = String::from("\n");
    for y in 0..=map_size.y {
        for x in 0..=map_size.x {
            match tiles.contains(&IVec2::new(x, y)) {
                true => {
                    write!(&mut output, "#")?;
                }
                false => {
                    write!(&mut output, ".",)?;
                }
            }
        }
        writeln!(&mut output)?;
    }
    Ok(output)
}

#[tracing::instrument(skip(input))]
pub fn process(input: &str, grid_size: IVec2) -> miette::Result<String> {
    let (_input, falling_bytes) = parse(input).map_err(|e| miette!("parse failed {}", e))?;

    let mut ns = grid_size.x as usize; // heuristic
    let idx = loop {
        let end = falling_bytes.len().min(ns);
        let result = dijkstra(
            &IVec2::ZERO,
            |position| {
                DIRECTIONS
                    .iter()
                    .filter_map(|direction| {
                        let next_position = position + direction;
                        if !(0..=grid_size.x).contains(&next_position.x)
                            || !(0..=grid_size.y).contains(&next_position.y)
                        {
                            return None;
                        }

                        if falling_bytes[0..end].contains(&next_position).not() {
                            Some((next_position, 1usize))
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>()
            },
            |&position| position == grid_size,
        );

        if result.is_none() {
            break ns;
        }

        ns += 1;
    };

    Ok(format!(
        "{},{}",
        falling_bytes[idx - 1].x,
        falling_bytes[idx - 1].y
    ))
}

#[cfg(test)]
mod day_18_part2 {
    use super::*;

    #[test_log::test]
    fn example() -> miette::Result<()> {
        let input = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";
        assert_eq!("6,1", process(input, IVec2::splat(6))?);
        Ok(())
    }

    #[test_log::test]
    fn input() -> miette::Result<()> {
        let input = include_str!("../inputs/input.txt");
        assert_eq!("20,12", process(input, IVec2::splat(70))?);
        Ok(())
    }
}
