use crate::custom_error::AocError;
use crate::nom_locate_utils::*;

use glam::IVec2;
use itertools::Itertools;
use nom::{
    branch::alt, bytes::complete::tag, character::complete::multispace0, combinator::all_consuming,
    multi::many1, sequence::terminated, IResult, Parser,
};
use std::collections::{HashMap, HashSet};
use tracing::{debug, info};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Eq, PartialEq)]
enum TileType {
    Empty,
    ForwardMirror,
    BackwardMirror,
    HorizontalSplitter,
    VerticalSplitter,
}

#[derive(Debug, Eq, PartialEq)]
struct TileInfo<'a> {
    pub span: SpanIVec2<'a>,
    pub space_type: TileType,
}

fn parse_grid(input: Span) -> IResult<Span, HashMap<IVec2, TileType>> {
    let (input, output) = all_consuming(many1(terminated(
        alt((
            tag(".").map(with_xy).map(|span| TileInfo {
                span,
                space_type: TileType::Empty,
            }),
            tag("/").map(with_xy).map(|span| TileInfo {
                span,
                space_type: TileType::ForwardMirror,
            }),
            tag("\\").map(with_xy).map(|span| TileInfo {
                span,
                space_type: TileType::BackwardMirror,
            }),
            tag("|").map(with_xy).map(|span| TileInfo {
                span,
                space_type: TileType::VerticalSplitter,
            }),
            tag("-").map(with_xy).map(|span| TileInfo {
                span,
                space_type: TileType::HorizontalSplitter,
            }),
        )),
        multispace0,
    )))(input)?;

    Ok((
        input,
        output
            .into_iter()
            .filter_map(|space_info| Some((space_info.span.extra, space_info.space_type)))
            .collect(),
    ))
}

fn display_light_bounces(
    input: &str,
    grid: &HashMap<IVec2, TileType>,
    energized: &HashMap<IVec2, HashSet<Direction>>,
) {
    let rows = input.lines().count();
    let cols = input.lines().next().unwrap().chars().count();
    let mut energized_grid = vec![vec!['.'; cols]; rows];
    for (pos, tile_type) in grid.iter() {
        let c = match energized.get(pos) {
            Some(directions) if grid.get(pos).unwrap() == &TileType::Empty => {
                if directions.len() > 1 {
                    directions.len().to_string().chars().next().unwrap()
                } else {
                    match directions.iter().next().unwrap() {
                        Direction::Up => '^',
                        Direction::Down => 'v',
                        Direction::Left => '<',
                        Direction::Right => '>',
                    }
                }
            }
            Some(_) | None => match tile_type {
                TileType::Empty => '.',
                TileType::ForwardMirror => '/',
                TileType::BackwardMirror => '\\',
                TileType::HorizontalSplitter => '-',
                TileType::VerticalSplitter => '|',
            },
        };
        energized_grid[pos.y as usize][pos.x as usize] = c
    }

    info!(
        "Light bounces\n{}",
        energized_grid
            .iter()
            .map(|row| row.iter().join(""))
            .join("\n")
    );
}

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<u64, AocError> {
    let (_, grid) = parse_grid(Span::new(input)).expect("a valid grid parse");

    let mut energized: HashMap<IVec2, HashSet<Direction>> = HashMap::new();
    let mut stack = vec![(IVec2::new(0, 0), Direction::Right)];

    while let Some((tile_pos, direction)) = stack.pop() {
        let tile_type = match grid.get(&tile_pos) {
            Some(tyle_type) => tyle_type,
            None => continue,
        };
        debug!(tile = ?tile_type, pos = %tile_pos, direction = ?direction, "checking");

        match energized.get_mut(&tile_pos) {
            Some(directions) => {
                if directions.contains(&direction) {
                    continue;
                }
                directions.insert(direction.clone());
            }
            None => {
                let mut directions = HashSet::new();
                directions.insert(direction.clone());
                energized.insert(tile_pos, directions);
            }
        }

        let up = tile_pos + IVec2::new(0, -1);
        let down = tile_pos + IVec2::new(0, 1);
        let left = tile_pos + IVec2::new(-1, 0);
        let right = tile_pos + IVec2::new(1, 0);

        match (tile_type, direction) {
            (TileType::Empty, Direction::Up) => stack.push((up, Direction::Up)),
            (TileType::Empty, Direction::Down) => stack.push((down, Direction::Down)),
            (TileType::Empty, Direction::Left) => stack.push((left, Direction::Left)),
            (TileType::Empty, Direction::Right) => stack.push((right, Direction::Right)),

            (TileType::ForwardMirror, Direction::Up) => stack.push((right, Direction::Right)),
            (TileType::ForwardMirror, Direction::Down) => stack.push((left, Direction::Left)),
            (TileType::ForwardMirror, Direction::Left) => stack.push((down, Direction::Down)),
            (TileType::ForwardMirror, Direction::Right) => stack.push((up, Direction::Up)),

            (TileType::BackwardMirror, Direction::Up) => stack.push((left, Direction::Left)),
            (TileType::BackwardMirror, Direction::Down) => stack.push((right, Direction::Right)),
            (TileType::BackwardMirror, Direction::Left) => stack.push((up, Direction::Up)),
            (TileType::BackwardMirror, Direction::Right) => stack.push((down, Direction::Down)),

            (TileType::HorizontalSplitter, Direction::Up | Direction::Down) => {
                stack.push((left, Direction::Left));
                stack.push((right, Direction::Right));
            }
            (TileType::HorizontalSplitter, Direction::Left) => stack.push((left, Direction::Left)),
            (TileType::HorizontalSplitter, Direction::Right) => {
                stack.push((right, Direction::Right))
            }

            (TileType::VerticalSplitter, Direction::Up) => stack.push((up, Direction::Up)),
            (TileType::VerticalSplitter, Direction::Down) => stack.push((down, Direction::Down)),
            (TileType::VerticalSplitter, Direction::Left | Direction::Right) => {
                stack.push((up, Direction::Up));
                stack.push((down, Direction::Down));
            }
        }
    }

    display_light_bounces(input, &grid, &energized);

    Ok(energized.len() as u64)
}

#[cfg(test)]
mod day_16_part1 {
    use super::*;

    #[test_log::test]
    fn example() -> miette::Result<()> {
        let input = r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#;
        assert_eq!(46, process(input)?);
        Ok(())
    }

    #[test_log::test]
    fn input1() -> miette::Result<()> {
        let input = include_str!("../inputs/input1.txt");
        assert_eq!(6978, process(input)?);
        Ok(())
    }
}
