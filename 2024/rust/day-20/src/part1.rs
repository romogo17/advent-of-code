use std::{collections::HashSet, ops::Not};

use glam::IVec2;
use miette::miette;
use nom::{
    character::complete::{line_ending, one_of},
    combinator::{all_consuming, opt},
    multi::{many1, separated_list1},
    IResult,
};
use nom_locate::LocatedSpan;
use pathfinding::prelude::*;
use tracing::debug;

type Span<'a> = LocatedSpan<&'a str>;

#[derive(Debug)]
struct Maze {
    start: IVec2,
    end: IVec2,
    walls: HashSet<IVec2>,
}

fn maze_tile(input: Span) -> IResult<Span, (IVec2, char)> {
    let y = input.location_line();
    let x = input.get_column();
    let (input, tile) = one_of(".#SE")(input)?;

    Ok((input, (IVec2::new(x as i32 - 1, y as i32 - 1), tile)))
}

fn parse(input: Span) -> IResult<Span, Maze> {
    let (input, tiles) = separated_list1(line_ending, many1(maze_tile))(input)?;
    let (input, _) = opt(line_ending)(input)?;

    let (start_pos, _) = tiles
        .iter()
        .flatten()
        .find(|(_, v)| v == &'S')
        .cloned()
        .expect("should have a starting position");
    let (end_pos, _) = tiles
        .iter()
        .flatten()
        .find(|(_, v)| v == &'E')
        .cloned()
        .expect("should have an ending position");
    let wall_pos = tiles
        .into_iter()
        .flatten()
        .filter_map(|(pos, value)| (value == '#').then_some(pos))
        .collect::<HashSet<IVec2>>();

    Ok((
        input,
        Maze {
            start: start_pos,
            end: end_pos,
            walls: wall_pos,
        },
    ))
}

const DIRECTIONS: [IVec2; 4] = [IVec2::X, IVec2::Y, IVec2::NEG_X, IVec2::NEG_Y];

#[tracing::instrument(skip(input))]
pub fn process(input: &str, savings_threshold: i32) -> miette::Result<u32> {
    let (_input, Maze { start, end, walls }) =
        all_consuming(parse)(Span::new(input)).map_err(|e| miette!("parse failed {}", e))?;

    let x_max = walls.iter().map(|pos| pos.x).max().unwrap();
    let y_max = walls.iter().map(|pos| pos.y).max().unwrap();

    let path_wo_cheats = dijkstra(
        &start,
        |position| {
            DIRECTIONS
                .iter()
                .filter_map(|direction| {
                    let next_position = position + direction;
                    walls
                        .contains(&next_position)
                        .not()
                        .then_some((next_position, 1))
                })
                .collect::<Vec<_>>()
        },
        |&position| position == end,
    )
    .expect("a valid path");
    debug!(?path_wo_cheats);

    // run a pathfind ith one missing wall, for walls with two empty sides
    let result = walls
        .iter()
        .filter(|wall| {
            DIRECTIONS
                .iter()
                .filter(|direction| {
                    let next_position = **wall + **direction;
                    (0..x_max).contains(&next_position.x)
                        && (0..y_max).contains(&next_position.y)
                        && walls.contains(&next_position).not()
                })
                .count()
                == 2
        })
        .filter_map(|wall| {
            dijkstra(
                &start,
                |position| {
                    DIRECTIONS
                        .iter()
                        .filter_map(|direction| {
                            let next_position = position + direction;
                            (next_position == *wall || walls.contains(&next_position).not())
                                .then_some((next_position, 1))
                        })
                        .collect::<Vec<_>>()
                },
                |&position| position == end,
            )
            .map(|(path, cost)| (path, cost, wall))
        })
        .map(|(_path, cost, wall)| {
            let cheat_gain = path_wo_cheats.1 - cost;
            debug!(
                "cost w/o cheats {}, cost with cheat {} (skipping wall {}), savings {}",
                path_wo_cheats.1, cost, wall, cheat_gain
            );

            cheat_gain
        })
        .filter(|cost| cost >= &savings_threshold)
        .count();

    Ok(result as u32)
}

#[cfg(test)]
mod day_20_part1 {
    use super::*;

    #[test_log::test]
    fn example() -> miette::Result<()> {
        let input = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";
        assert_eq!(5, process(input, 15)?);
        Ok(())
    }

    #[test_log::test]
    fn input() -> miette::Result<()> {
        let input = include_str!("../inputs/input.txt");
        assert_eq!(1365, process(input, 100)?);
        Ok(())
    }
}
