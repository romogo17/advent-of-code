use crate::custom_error::AocError;
use crate::nom_locate_utils::*;

use glam::IVec2;
use nom::{
    branch::alt,
    bytes::complete::{is_a, tag},
    combinator::{all_consuming, opt},
    multi::many1,
    sequence::delimited,
    IResult, Parser,
};
use petgraph::{algo, prelude::*};
use std::collections::HashMap;
use tracing::debug;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn next(&self, from: &IVec2) -> IVec2 {
        *from
            + match self {
                Direction::Up => IVec2::NEG_Y,
                Direction::Down => IVec2::Y,
                Direction::Left => IVec2::NEG_X,
                Direction::Right => IVec2::X,
            }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum MapTile {
    Ground,
    Slope(Direction),
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
struct MapTileInfo<'a> {
    span: SpanIVec2<'a>,
    tile_type: MapTile,
}

fn parse_grid(input: Span) -> IResult<Span, HashMap<IVec2, MapTile>> {
    let (input, output) = all_consuming(many1(delimited(
        opt(is_a("#\n")),
        alt((
            tag("^").map(with_xy).map(|span| MapTileInfo {
                span,
                tile_type: MapTile::Slope(Direction::Up),
            }),
            tag("v").map(with_xy).map(|span| MapTileInfo {
                span,
                tile_type: MapTile::Slope(Direction::Down),
            }),
            tag(">").map(with_xy).map(|span| MapTileInfo {
                span,
                tile_type: MapTile::Slope(Direction::Right),
            }),
            tag("<").map(with_xy).map(|span| MapTileInfo {
                span,
                tile_type: MapTile::Slope(Direction::Left),
            }),
            tag(".").map(with_xy).map(|span| MapTileInfo {
                span,
                tile_type: MapTile::Ground,
            }),
        )),
        opt(is_a("#\n")),
    )))(input)?;

    Ok((
        input,
        output
            .into_iter()
            .filter_map(|map_tile_info| Some((map_tile_info.span.extra, map_tile_info.tile_type)))
            .collect(),
    ))
}

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<u64, AocError> {
    let (_, grid) = parse_grid(Span::new(input)).expect("a valid grid parse");

    let start = grid
        .iter()
        .min_by_key(|(pos, _tile)| pos.y)
        .expect("should have a grid start")
        .0;
    let end = grid
        .iter()
        .max_by_key(|(pos, _tile)| pos.y)
        .expect("should have a grid start")
        .0;
    debug!(?start, ?end, "path edges");

    let mut graph = DiGraph::<(&IVec2, &MapTile), u32>::new();

    let node_idx_map: HashMap<&IVec2, NodeIndex> = grid
        .iter()
        .map(|(pos, tile)| (pos, graph.add_node((pos, tile))))
        .collect();

    grid.iter()
        .flat_map(|(pos, _tile)| {
            vec![
                Direction::Up,
                Direction::Down,
                Direction::Left,
                Direction::Right,
            ]
            .into_iter()
            .filter_map(|dir| {
                let next_pos = dir.next(pos);
                grid.contains_key(&next_pos)
                    .then(|| (node_idx_map[pos], node_idx_map[&next_pos], 1))
            })
        })
        .for_each(|(a, b, w)| {
            graph.add_edge(a, b, w);
        });

    let paths = algo::all_simple_paths::<Vec<_>, _>(
        &graph,
        node_idx_map[&start],
        node_idx_map[&end],
        0,
        None,
    );

    let longest_path_len = paths.max_by(|a, b| a.len().cmp(&b.len())).unwrap().len();

    // step count is the number of edges, not nodes
    Ok((longest_path_len - 1) as u64)
}

#[cfg(test)]
mod day_23_part2 {
    use super::*;

    #[test_log::test]
    fn example() -> miette::Result<()> {
        let input = "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";
        assert_eq!(154, process(input)?);
        Ok(())
    }
}
