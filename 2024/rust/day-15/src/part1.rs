use std::{
    collections::HashMap,
    fmt::{self, Display, Write},
};

use glam::IVec2;
use miette::miette;
use nom::{
    branch::alt,
    bytes::complete::is_a,
    character::complete::{self, line_ending, multispace1},
    combinator::{opt, value},
    multi::{many1, separated_list1},
    sequence::preceded,
    IResult,
};
use nom_locate::LocatedSpan;
use tracing::debug;

pub type Span<'a> = LocatedSpan<&'a str>;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Tile {
    Wall,
    Box,
    Robot,
}

impl Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match &self {
                Tile::Wall => "#",
                Tile::Box => "O",
                Tile::Robot => "@",
            }
        )
    }
}

fn map_to_string(tiles: &HashMap<IVec2, Tile>) -> Result<String, fmt::Error> {
    let map_size = IVec2::new(
        tiles.keys().map(|pos| pos.x).max().unwrap(),
        tiles.keys().map(|pos| pos.y).max().unwrap(),
    );
    let mut output = String::from("\n");
    for y in 0..=map_size.y {
        for x in 0..=map_size.x {
            match tiles.get(&IVec2::new(x, y)) {
                Some(tile) => {
                    write!(&mut output, "{tile}")?;
                }
                None => {
                    write!(&mut output, ".",)?;
                }
            }
        }
        writeln!(&mut output)?;
    }
    Ok(output)
}

fn parse_tile(input: Span) -> IResult<Span, (IVec2, Tile)> {
    let x = input.get_column();
    let y = input.location_line();
    let (input, tile) = alt((
        value(Tile::Wall, complete::char('#')),
        value(Tile::Box, complete::char('O')),
        value(Tile::Robot, complete::char('@')),
    ))(input)?;
    Ok((input, (IVec2::new(x as i32 - 1, y as i32 - 1), tile)))
}

fn parse(input: Span) -> IResult<Span, (HashMap<IVec2, Tile>, Vec<IVec2>)> {
    let (input, tiles) =
        separated_list1(line_ending, many1(preceded(opt(is_a(".")), parse_tile)))(input)?;

    let hashmap = tiles.into_iter().flatten().collect();

    let (input, directions) = preceded(
        multispace1,
        separated_list1(
            multispace1,
            many1(alt((
                value(IVec2::NEG_Y, complete::char('^')),
                value(IVec2::Y, complete::char('v')),
                value(IVec2::X, complete::char('>')),
                value(IVec2::NEG_X, complete::char('<')),
            ))),
        ),
    )(input)?;
    let directions = directions.into_iter().flatten().collect();

    Ok((input, (hashmap, directions)))
}

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<u32> {
    let (_input, (mut map, directions)) =
        parse(Span::new(input)).map_err(|e| miette!("parse failed {}", e))?;

    debug!("Starting map {}", map_to_string(&map).unwrap());

    for direction in directions {
        let robot_pos = map
            .iter()
            .find(|(_, tile)| tile == &&Tile::Robot)
            .expect("a robot")
            .0
            .clone();

        let next_pos = robot_pos + direction;
        let Some(next) = map.get(&next_pos) else {
            // empty tile, bot can move
            let robot = map
                .remove(&robot_pos)
                .expect("robot to exist in the map when removing");
            map.insert(next_pos, robot);

            debug!(
                "move={}, map after empty space {}",
                direction,
                map_to_string(&map).unwrap()
            );
            continue;
        };

        match next {
            Tile::Wall => {
                debug!(
                    "move={}, map after wall {}",
                    direction,
                    map_to_string(&map).unwrap()
                );
                continue;
            }
            Tile::Box => {
                // check all tiles until wall or space
                let mut items = vec![next_pos];
                while Some(&Tile::Box) == map.get(&(items.iter().last().unwrap() + direction)) {
                    items.push(items.iter().last().unwrap() + direction);
                }

                let Some(_next) = map.get(&(items.iter().last().unwrap() + direction)) else {
                    // bot and next item can move
                    // instead of moving all items, we move the robot and the next tile to the available position
                    let robot = map
                        .remove(&robot_pos)
                        .expect("robot to exist in the map when removing");
                    let mut it = items.iter();

                    // move the robot to the next position
                    let next_tile_pos = it.next().unwrap();
                    let tile = map
                        .remove(next_tile_pos)
                        .expect("a tile to exist in the map when removing");
                    map.insert(*next_tile_pos, robot);

                    // move the tile to the available position
                    match it.last() {
                        Some(last_tile_pos) => {
                            map.insert(*last_tile_pos + direction, tile);
                        }
                        None => {
                            map.insert(*next_tile_pos + direction, tile);
                        }
                    }

                    debug!(
                        "move={}, map after box {}",
                        direction,
                        map_to_string(&map).unwrap()
                    );
                    continue;
                };
            }
            Tile::Robot => {
                unreachable!("there should only be one robot");
            }
        }
    }

    let result: i32 = map
        .iter()
        .filter(|(_, tile)| tile == &&Tile::Box)
        .map(|(pos, _)| 100 * pos.y + pos.x)
        .sum();

    Ok(result as u32)
}

#[cfg(test)]
mod day_15_part1 {
    use super::*;

    #[test_log::test]
    fn small_example() -> miette::Result<()> {
        let input = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";
        assert_eq!(2028, process(input)?);
        Ok(())
    }

    #[test_log::test]
    fn big_example() -> miette::Result<()> {
        let input = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";
        assert_eq!(10092, process(input)?);
        Ok(())
    }

    #[test_log::test]
    fn input() -> miette::Result<()> {
        let input = include_str!("../inputs/input.txt");
        assert_eq!(1437174, process(input)?);
        Ok(())
    }
}
