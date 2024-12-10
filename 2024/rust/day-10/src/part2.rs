use std::{collections::HashMap, usize};

use glam::IVec2;
use miette::miette;
use nom::{
    character::complete::{line_ending, satisfy},
    multi::{many1, separated_list1},
    IResult,
};
use nom_locate::LocatedSpan;
use pathfinding::prelude::count_paths;
use tracing::debug;

type Span<'a> = LocatedSpan<&'a str>;
const DIRECTIONS: [IVec2; 4] = [IVec2::X, IVec2::NEG_X, IVec2::Y, IVec2::NEG_Y];

fn num_pos(input: Span) -> IResult<Span, (IVec2, u32)> {
    let x = input.get_column() as i32 - 1;
    let y = input.location_line() as i32 - 1;
    let (input, digit) = satisfy(|c| c.is_numeric())(input)?;
    Ok((input, (IVec2::new(x, y), digit.to_digit(10).unwrap())))
}

fn parse(input: Span) -> IResult<Span, HashMap<IVec2, u32>> {
    let (input, lines) = separated_list1(line_ending, many1(num_pos))(input)?;
    Ok((input, lines.iter().flatten().cloned().collect()))
}

fn search_trail(map: &HashMap<IVec2, u32>, trailhead: IVec2) -> HashMap<IVec2, u32> {
    let mut trail = HashMap::from([(trailhead, 0u32)]);
    let mut stack: Vec<(IVec2, i32)> = DIRECTIONS
        .iter()
        .map(|dir| (trailhead + dir, 0i32))
        .collect();

    while let Some((pos, from_value)) = stack.pop() {
        if let Some(value) = map.get(&pos) {
            let diff = *value as i32 - from_value;
            if diff == 1 {
                trail.entry(pos).or_insert(*value);
                for dir in DIRECTIONS.iter() {
                    if !trail.contains_key(&(pos + dir)) {
                        stack.push((pos + dir, *value as i32));
                    }
                }
            }
        }
    }

    trail
}

fn trail_rating(map: &HashMap<IVec2, u32>, trailhead: &IVec2, endings: &[IVec2]) -> u32 {
    endings
        .iter()
        .map(|ending_pos| {
            count_paths(
                *trailhead,
                |pos| {
                    DIRECTIONS
                        .iter()
                        .zip(std::iter::repeat(*pos))
                        .map(|(dir, pos)| (dir + pos, pos))
                        .filter(|(new_pos, from_pos)| {
                            map.get(new_pos).is_some_and(|h| {
                                let current_height = map.get(from_pos).unwrap();

                                *h == current_height + 1
                            })
                        })
                        .map(|(new, _)| new)
                },
                |c| c == ending_pos,
            )
        })
        .sum::<usize>() as u32
}

fn print_trail(trail: &HashMap<IVec2, u32>, height: usize, width: usize) {
    for y in 0..height {
        for x in 0..width {
            let pos = IVec2::new(x as i32, y as i32);
            if let Some(value) = trail.get(&pos) {
                print!("{}", value);
            } else {
                print!(".");
            }
        }
        println!();
    }
}

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<u32> {
    let height = input.lines().count();
    let width = input.lines().next().unwrap().len();

    let (_input, map) = parse(Span::new(input)).map_err(|e| miette!("parse failed {}", e))?;

    let trailheads = map
        .iter()
        .filter(|(_, &v)| v == 0)
        .map(|(&k, _)| k)
        .collect::<Vec<_>>();
    debug!("trailheads: {:?}", trailheads);

    let result: u32 = trailheads
        .iter()
        .map(|trailhead| {
            let trail = search_trail(&map, *trailhead);
            print_trail(&trail, height, width);

            let endings = trail
                .iter()
                .filter(|(_, value)| **value == 9u32)
                .map(|(pos, _)| *pos)
                .collect::<Vec<_>>();

            let rating = trail_rating(&map, trailhead, &endings);

            debug!("trail rating: {}", rating);

            rating
        })
        .sum();

    Ok(result)
}

#[cfg(test)]
mod day_10_part2 {
    use super::*;

    #[test_log::test]
    fn example() -> miette::Result<()> {
        let input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
        assert_eq!(81, process(input)?);
        Ok(())
    }

    #[test_log::test]
    fn input() -> miette::Result<()> {
        let input = include_str!("../inputs/input.txt");
        assert_eq!(1463, process(input)?);
        Ok(())
    }
}
