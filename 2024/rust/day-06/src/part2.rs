use std::collections::{HashMap, HashSet};

use glam::IVec2;
use itertools::Itertools;
use miette::miette;
use nom::{
    character::complete::{line_ending, one_of},
    multi::{many1, separated_list1},
    IResult,
};
use nom_locate::LocatedSpan;
use tracing::debug;

pub type Span<'a> = LocatedSpan<&'a str>;

fn token(input: Span) -> IResult<Span, (IVec2, char)> {
    let x = input.get_column();
    let y = input.location_line();

    let (input, token) = one_of(".#^")(input)?;
    Ok((input, (IVec2::new(x as i32 - 1, y as i32 - 1), token)))
}

fn parse(input: Span) -> IResult<Span, ((IVec2, char), HashMap<IVec2, char>)> {
    let (input, tokens) = separated_list1(line_ending, many1(token))(input)?;

    let guard = tokens
        .iter()
        .flatten()
        .find(|(_, c)| c == &'^')
        .cloned()
        .expect("should have a guard token");

    let obstacles = tokens
        .into_iter()
        .flatten()
        .filter(|(_, c)| c == &'#')
        .collect::<HashMap<IVec2, char>>();

    Ok((input, (guard, obstacles)))
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn turn_right(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    fn next_pos(&self) -> IVec2 {
        match self {
            Direction::North => IVec2::NEG_Y,
            Direction::East => IVec2::X,
            Direction::South => IVec2::Y,
            Direction::West => IVec2::NEG_X,
        }
    }
}

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<u32> {
    let (_input, ((mut guard_pos, _), obstacles)) =
        parse(Span::new(input)).map_err(|e| miette!("parse failed {}", e))?;
    let initial_guard_pos = guard_pos.clone();
    debug!(?guard_pos, ?obstacles);

    let (x_min, x_max) = obstacles
        .iter()
        .map(|(pos, _)| pos.x)
        .minmax()
        .into_option()
        .unwrap();

    let (y_min, y_max) = obstacles
        .iter()
        .map(|(pos, _)| pos.y)
        .minmax()
        .into_option()
        .unwrap();

    let mut guard_dir = Direction::North;
    let mut visited = HashSet::from([guard_pos]);

    loop {
        let next_pos = guard_pos + guard_dir.next_pos();

        if obstacles.get(&next_pos).is_some() {
            // if there's a wall in the next position, turn right
            guard_dir = guard_dir.turn_right();
        } else if (x_min..=x_max).contains(&next_pos.x) && (y_min..=y_max).contains(&next_pos.y) {
            // otherwise, if the next position is within bounds, continue in the same direction
            guard_pos = next_pos;
            visited.insert(guard_pos);
        } else {
            break;
        }
    }
    debug!(?visited);

    // we can't place an obstacle in the initial guard position
    visited.remove(&initial_guard_pos);

    // place an obstacle in each position visited to check which ones form a loop
    let viable_obstacles = visited
        .iter()
        .filter(|new_obstacle| {
            let mut guard_pos = initial_guard_pos.clone();
            let mut guard_dir = Direction::North;
            let mut visited = HashSet::from([(guard_pos, guard_dir.clone())]);

            loop {
                let next_pos = guard_pos + guard_dir.next_pos();

                if obstacles.get(&next_pos).is_some() || &&next_pos == new_obstacle {
                    // if there's a wall in the next position, turn right
                    guard_dir = guard_dir.turn_right();
                    continue;
                }

                if visited.get(&(next_pos, guard_dir.clone())).is_some() {
                    // if we've already visited this position in this direction, we've found a loop
                    break true;
                } else if (x_min..=x_max).contains(&next_pos.x)
                    && (y_min..=y_max).contains(&next_pos.y)
                {
                    // otherwise, if the next position is within bounds, continue in the same direction
                    guard_pos = next_pos;
                    visited.insert((guard_pos, guard_dir.clone()));
                    continue;
                } else {
                    break false;
                }
            }
        })
        .collect::<Vec<&IVec2>>();

    Ok((viable_obstacles.len()) as u32)
}

#[cfg(test)]
mod day_06_part2 {
    use super::*;

    #[test_log::test]
    fn example() -> miette::Result<()> {
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        assert_eq!(6, process(input)?);
        Ok(())
    }

    #[test_log::test]
    fn input() -> miette::Result<()> {
        let input = include_str!("../inputs/input.txt");
        assert_eq!(1602, process(input)?);
        Ok(())
    }
}
