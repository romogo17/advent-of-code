use std::{
    collections::{HashMap, HashSet},
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
    BoxLeft,
    BoxRight,
    Robot,
}

impl Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match &self {
                Tile::Wall => "#",
                Tile::BoxLeft => "[",
                Tile::BoxRight => "]",
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
        value(Tile::BoxLeft, complete::char('[')),
        value(Tile::BoxRight, complete::char(']')),
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

fn vertical(
    tile: &Tile,
    map: &mut HashMap<IVec2, Tile>,
    next_pos: IVec2,
    direction: IVec2,
    robot_pos: IVec2,
) {
    let mut seen_items: HashSet<IVec2> = HashSet::from([robot_pos]);
    let mut next_items: Vec<IVec2> = match tile {
        Tile::BoxLeft => {
            vec![next_pos, next_pos + IVec2::X]
        }
        Tile::BoxRight => {
            vec![next_pos + IVec2::NEG_X, next_pos]
        }
        _ => unreachable!(""),
    };

    while !next_items.is_empty() {
        let mut new_items: HashSet<IVec2> = HashSet::default();
        for item in &next_items {
            // check in direction
            let next_pos = *item + direction;
            match map.get(&next_pos) {
                Some(Tile::Wall) => {
                    return;
                }
                Some(Tile::BoxLeft) => {
                    // get box right
                    new_items.insert(next_pos);
                    new_items.insert(next_pos + IVec2::X);
                }
                Some(Tile::BoxRight) => {
                    // get box left
                    new_items.insert(next_pos);
                    new_items.insert(next_pos + IVec2::NEG_X);
                }
                Some(_) => {
                    unreachable!("");
                }
                None => {}
            }
        }
        for item in &next_items {
            seen_items.insert(*item);
        }
        next_items = vec![];
        for item in &new_items {
            next_items.push(*item);
        }
    }

    let mut items: Vec<IVec2> = seen_items.into_iter().collect();
    items.sort_by(|a, b| {
        if direction.y > 0 {
            b.y.cmp(&a.y)
        } else {
            a.y.cmp(&b.y)
        }
    });

    for item in items {
        let v = map.remove(&item).unwrap();
        map.insert(item + direction, v);
    }
}

fn horizontal(map: &mut HashMap<IVec2, Tile>, next_pos: IVec2, direction: IVec2, robot_pos: IVec2) {
    let mut items = vec![next_pos];
    while [Some(&Tile::BoxLeft), Some(&Tile::BoxRight)]
        .contains(&map.get(&(items.iter().last().unwrap() + direction)))
    {
        items.push(items.iter().last().unwrap() + direction);
    }

    let Some(_next) = map.get(&(items.iter().last().unwrap() + direction)) else {
        // bot *and* next item can move
        let mut last_item = map
            .remove(&robot_pos)
            .expect("robot to exist when removing");
        for item_location in &items {
            let item_to_insert = last_item;
            // remove the item
            last_item = map
                .remove(&item_location)
                .expect("tile to exist when removing");
            map.insert(*item_location, item_to_insert);
        }

        let location = items.iter().last().unwrap();
        map.insert(*location + direction, last_item);

        return;
    };
}

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<u32> {
    let new_input = input
        .chars()
        .map(|c| match c {
            '#' => "##".to_string(),
            'O' => "[]".to_string(),
            '.' => "..".to_string(),
            '@' => "@.".to_string(),
            other => other.to_string(),
        })
        .collect::<String>();
    let (_input, (mut map, directions)) =
        parse(Span::new(&new_input)).map_err(|e| miette!("parse failed {}", e))?;

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
            Tile::BoxLeft => {
                if direction.x == 0 {
                    // vertical movement
                    vertical(&Tile::BoxLeft, &mut map, next_pos, direction, robot_pos)
                } else {
                    // horizontal movement to the right
                    horizontal(&mut map, next_pos, direction, robot_pos)
                }
                debug!(
                    "move={}, map after box-left {}",
                    direction,
                    map_to_string(&map).unwrap()
                );
            }
            Tile::BoxRight => {
                if direction.x == 0 {
                    // vertical movement
                    vertical(&Tile::BoxRight, &mut map, next_pos, direction, robot_pos)
                } else {
                    // horizontal movement to the left
                    horizontal(&mut map, next_pos, direction, robot_pos)
                }
                debug!(
                    "move={}, map after box-right {}",
                    direction,
                    map_to_string(&map).unwrap()
                );
            }
            Tile::Robot => {
                unreachable!("there should only be one robot");
            }
        }
    }

    let result: i32 = map
        .iter()
        .filter(|(_, tile)| tile == &&Tile::BoxLeft)
        .map(|(pos, _)| 100 * pos.y + pos.x)
        .sum();

    Ok(result as u32)
}

#[cfg(test)]
mod day_15_part2 {
    use super::*;

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
        assert_eq!(9021, process(input)?);
        Ok(())
    }

    #[test_log::test]
    fn input() -> miette::Result<()> {
        let input = include_str!("../inputs/input.txt");
        assert_eq!(1437468, process(input)?);
        Ok(())
    }
}
