use std::collections::HashSet;

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

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<u32> {
    let (_input, Maze { start, end, walls }) =
        all_consuming(parse)(Span::new(input)).map_err(|e| miette!("parse failed {}", e))?;

    let (result, _cost) = astar_bag(
        &(start, IVec2::X),
        |(position, direction)| {
            let next_position = position + direction;
            if walls.contains(&next_position) {
                // if there's a wall in the next position, we can only turn left or right
                vec![
                    ((*position, direction.perp()), 1000),
                    ((*position, -direction.perp()), 1000),
                ]
            } else {
                // if there's no wall, we can either continue on the same direction or turn
                vec![
                    ((next_position, *direction), 1),
                    ((*position, direction.perp()), 1000),
                    ((*position, -direction.perp()), 1000),
                ]
            }
        },
        |_| 0,
        |&(position, _direction)| position == end,
    )
    .expect("valid path(s)");

    let result = result
        .flat_map(|path| path.into_iter().map(|(position, _)| position))
        .collect::<HashSet<IVec2>>()
        .len();

    Ok(result as u32)
}

#[cfg(test)]
mod day_16_part2 {
    use super::*;

    #[test_log::test]
    fn example1() -> miette::Result<()> {
        let input = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";
        assert_eq!(45, process(input)?);
        Ok(())
    }

    #[test_log::test]
    fn example2() -> miette::Result<()> {
        let input = "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";
        assert_eq!(64, process(input)?);
        Ok(())
    }

    #[test_log::test]
    fn input() -> miette::Result<()> {
        let input = include_str!("../inputs/input.txt");
        assert_eq!(479, process(input)?);
        Ok(())
    }
}
