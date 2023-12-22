use crate::custom_error::AocError;
use crate::nom_locate_utils::*;

use glam::IVec2;
use nom::{
    branch::alt, bytes::complete::tag, character::complete::multispace0, combinator::all_consuming,
    multi::many1, sequence::terminated, IResult, Parser,
};
use std::collections::{HashMap, HashSet};
use tracing::debug;

#[derive(Debug, Eq, PartialEq)]
enum MapTile {
    Start,
    GardenPlot,
    Rock,
}

#[derive(Debug, Eq, PartialEq)]
struct MapTileInfo<'a> {
    span: SpanIVec2<'a>,
    tile_type: MapTile,
}

fn parse_grid(input: Span) -> IResult<Span, HashMap<IVec2, MapTile>> {
    let (input, output) = all_consuming(many1(terminated(
        alt((
            tag(".").map(with_xy).map(|span| MapTileInfo {
                span,
                tile_type: MapTile::GardenPlot,
            }),
            tag("S").map(with_xy).map(|span| MapTileInfo {
                span,
                tile_type: MapTile::Start,
            }),
            tag("#").map(with_xy).map(|span| MapTileInfo {
                span,
                tile_type: MapTile::Rock,
            }),
        )),
        multispace0,
    )))(input)?;

    Ok((
        input,
        output
            .into_iter()
            .filter_map(|map_tile_info| Some((map_tile_info.span.extra, map_tile_info.tile_type)))
            .collect(),
    ))
}

#[tracing::instrument(skip(input, total_steps))]
pub fn process(input: &str, total_steps: u64) -> miette::Result<u64, AocError> {
    let (_, grid) = parse_grid(Span::new(input)).expect("a valid grid parse");
    let start = grid
        .iter()
        .find_map(|(pos, tile_type)| match tile_type {
            MapTile::Start => Some(pos),
            _ => None,
        })
        .expect("grid should have a start position");

    let mut steps_at: HashMap<u64, HashSet<IVec2>> =
        HashMap::from([(0u64, HashSet::from([*start]))]);
    for steps in 1..=total_steps {
        let mut pos_to_check = steps_at
            .get(&(steps - 1))
            .unwrap()
            .iter()
            .flat_map(|pos| {
                vec![
                    *pos + IVec2::X,
                    *pos + IVec2::NEG_X,
                    *pos + IVec2::Y,
                    *pos + IVec2::NEG_Y,
                ]
            })
            .collect::<Vec<IVec2>>();

        while let Some(pos) = pos_to_check.pop() {
            match grid.get(&pos) {
                Some(MapTile::Rock) | None => {}
                Some(MapTile::GardenPlot) | Some(MapTile::Start) => {
                    // debug!(?pos, ?steps);
                    steps_at
                        .entry(steps)
                        .and_modify(|pos_set| {
                            pos_set.insert(pos);
                        })
                        .or_insert(HashSet::from([pos]));
                }
            }
        }

        debug!(
            "there are {:?} after walking {} steps",
            steps_at.get(&steps).unwrap().len(),
            steps
        );
    }

    Ok(steps_at.get(&total_steps).unwrap().len() as u64)
}

#[cfg(test)]
mod day_21_part1 {
    use super::*;

    #[test_log::test]
    fn example() -> miette::Result<()> {
        let input = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";
        assert_eq!(16, process(input, 6u64)?);
        Ok(())
    }

    #[test_log::test]
    fn input1() -> miette::Result<()> {
        let input = include_str!("../inputs/input1.txt");
        assert_eq!(3646, process(input, 64u64)?);
        Ok(())
    }
}
