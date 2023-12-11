// use day_10::*;

use std::collections::HashMap;

use glam::IVec2;
use nom::{
    branch::alt, bytes::complete::tag, character::complete::multispace0, combinator::all_consuming,
    multi::many1, sequence::terminated, IResult, Parser,
};
use nom_locate::LocatedSpan;

#[derive(Debug, Eq, PartialEq)]
enum PipeType {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
    Start,
    Ground,
}

impl PipeType {
    fn is_pipe_connection_valid(&self, other: &PipeType, direction: &Direction) -> bool {
        match (self, direction) {
            (PipeType::Vertical, Direction::North) => match other {
                PipeType::Vertical => true,
                PipeType::SouthEast => true,
                PipeType::SouthWest => true,
                PipeType::Start => true,
                _ => false,
            },

            (PipeType::Vertical, Direction::South) => match other {
                PipeType::Vertical => true,
                PipeType::NorthEast => true,
                PipeType::NorthWest => true,
                PipeType::Start => true,
                _ => false,
            },

            (PipeType::Horizontal, Direction::West) => match other {
                PipeType::Horizontal => true,
                PipeType::NorthEast => true,
                PipeType::SouthEast => true,
                PipeType::Start => true,
                _ => false,
            },

            (PipeType::Horizontal, Direction::East) => match other {
                PipeType::Horizontal => true,
                PipeType::NorthWest => true,
                PipeType::SouthWest => true,
                PipeType::Start => true,
                _ => false,
            },

            (PipeType::NorthEast, Direction::North) => match other {
                PipeType::Vertical => true,
                PipeType::SouthEast => true,
                PipeType::SouthWest => true,
                PipeType::Start => true,
                _ => false,
            },

            (PipeType::NorthEast, Direction::East) => match other {
                PipeType::Horizontal => true,
                PipeType::NorthWest => true,
                PipeType::SouthWest => true,
                PipeType::Start => true,
                _ => false,
            },

            (PipeType::NorthWest, Direction::North) => match other {
                PipeType::Vertical => true,
                PipeType::SouthEast => true,
                PipeType::SouthWest => true,
                PipeType::Start => true,
                _ => false,
            },

            (PipeType::NorthWest, Direction::West) => match other {
                PipeType::Horizontal => true,
                PipeType::NorthEast => true,
                PipeType::SouthEast => true,
                PipeType::Start => true,
                _ => false,
            },

            (PipeType::SouthEast, Direction::South) => match other {
                PipeType::Vertical => true,
                PipeType::NorthEast => true,
                PipeType::NorthWest => true,
                PipeType::Start => true,
                _ => false,
            },

            (PipeType::SouthEast, Direction::East) => match other {
                PipeType::Horizontal => true,
                PipeType::NorthWest => true,
                PipeType::SouthWest => true,
                PipeType::Start => true,
                _ => false,
            },

            (PipeType::SouthWest, Direction::South) => match other {
                PipeType::Vertical => true,
                PipeType::NorthEast => true,
                PipeType::NorthWest => true,
                PipeType::Start => true,
                _ => false,
            },

            (PipeType::SouthWest, Direction::West) => match other {
                PipeType::Horizontal => true,
                PipeType::NorthEast => true,
                PipeType::SouthEast => true,
                _ => false,
            },

            (PipeType::Start, Direction::North) => match other {
                PipeType::Vertical => true,
                PipeType::SouthEast => true,
                PipeType::SouthWest => true,
                _ => false,
            },

            (PipeType::Start, Direction::South) => match other {
                PipeType::Vertical => true,
                PipeType::NorthEast => true,
                PipeType::NorthWest => true,
                _ => false,
            },

            (PipeType::Start, Direction::West) => match other {
                PipeType::Horizontal => true,
                PipeType::NorthEast => true,
                PipeType::SouthEast => true,
                _ => false,
            },

            (PipeType::Start, Direction::East) => match other {
                PipeType::Horizontal => true,
                PipeType::NorthWest => true,
                PipeType::SouthWest => true,
                _ => false,
            },

            _ => false,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Direction {
    North,
    South,
    West,
    East,
}

#[derive(Debug)]
struct PipeInfo<'a> {
    span: SpanIVec2<'a>,
    pipe_type: PipeType,
}

type Span<'a> = LocatedSpan<&'a str>;
type SpanIVec2<'a> = LocatedSpan<&'a str, IVec2>;

fn with_xy(span: Span) -> SpanIVec2 {
    // column and location line are 1-indexed
    let x = span.get_column() as i32 - 1;
    let y = span.location_line() as i32 - 1;
    span.map_extra(|_| IVec2::new(x, y))
}

fn parse_pipe_grid(input: Span) -> IResult<Span, HashMap<IVec2, PipeType>> {
    let (input, output) = all_consuming(many1(terminated(
        alt((
            tag("|").map(with_xy).map(|span| PipeInfo {
                span,
                pipe_type: PipeType::Vertical,
            }),
            tag("-").map(with_xy).map(|span| PipeInfo {
                span,
                pipe_type: PipeType::Horizontal,
            }),
            tag("L").map(with_xy).map(|span| PipeInfo {
                span,
                pipe_type: PipeType::NorthEast,
            }),
            tag("J").map(with_xy).map(|span| PipeInfo {
                span,
                pipe_type: PipeType::NorthWest,
            }),
            tag("F").map(with_xy).map(|span| PipeInfo {
                span,
                pipe_type: PipeType::SouthEast,
            }),
            tag("7").map(with_xy).map(|span| PipeInfo {
                span,
                pipe_type: PipeType::SouthWest,
            }),
            tag("S").map(with_xy).map(|span| PipeInfo {
                span,
                pipe_type: PipeType::Start,
            }),
            tag(".").map(with_xy).map(|span| PipeInfo {
                span,
                pipe_type: PipeType::Ground,
            }),
        )),
        multispace0,
    )))(input)?;

    Ok((
        input,
        output
            .into_iter()
            .filter_map(|pipe_info| {
                (pipe_info.pipe_type != PipeType::Ground)
                    .then_some((pipe_info.span.extra, pipe_info.pipe_type))
            })
            .collect(),
    ))
}

fn find_cycle(grid: &HashMap<IVec2, PipeType>, start_position: &IVec2) -> Vec<IVec2> {
    fn get_next_position(
        grid: &HashMap<IVec2, PipeType>,
        current_position: &IVec2,
        previous_position: Option<IVec2>,
    ) -> IVec2 {
        let pipe_type = grid
            .get(current_position)
            .expect("current position should always be in the grid");

        let next_positions: Vec<IVec2> = [
            (*current_position + IVec2::new(0, -1), Direction::North),
            (*current_position + IVec2::new(0, 1), Direction::South),
            (*current_position + IVec2::new(-1, 0), Direction::West),
            (*current_position + IVec2::new(1, 0), Direction::East),
        ]
        .iter()
        .filter_map(|(candidate_position, direction)| {
            if let Some(previous_position) = previous_position {
                if previous_position == *candidate_position {
                    return None;
                }
            }

            grid.get(candidate_position)
                .is_some_and(|candidate_pipe_type| {
                    pipe_type.is_pipe_connection_valid(candidate_pipe_type, direction)
                })
                .then_some(candidate_position.clone())
        })
        .collect();

        *next_positions.first().expect("no next positions found")
    }

    let mut previous_position: Option<IVec2> = None;
    let mut current_position = start_position.clone();
    let mut next_position: IVec2;
    let mut visited = vec![];

    loop {
        visited.push(current_position);

        next_position = get_next_position(grid, &current_position, previous_position);
        previous_position = Some(current_position.clone());
        current_position = next_position;

        if current_position == *start_position {
            break;
        }
    }

    visited
}

fn process(input: &str) -> u64 {
    let (_, grid) = parse_pipe_grid(Span::new(input)).expect("should parse a pipe grid");
    let start_position = grid
        .iter()
        .find_map(|(key, value)| (value == &PipeType::Start).then_some(key))
        .expect("grid should have a start position");

    let cycle = find_cycle(&grid, start_position);

    cycle.len() as u64 / 2
}

fn main() {
    let input = include_str!("../../inputs/input1.txt");
    let output = process(input);
    println!("Output is {output}");
}

#[cfg(test)]
mod day_10_part1 {
    use super::*;

    #[test]
    fn example1() {
        let input = "-L|F7
7S-7|
L|7||
-L-J|
L|-JF";
        let output = process(input);
        assert_eq!(output, 4);
    }

    #[test]
    fn example2() {
        let input = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";
        let output = process(input);
        assert_eq!(output, 8);
    }

    // #[test]
    // fn input1() {
    //     let input = include_str!("../../inputs/input1.txt");
    //     let output = process(input);
    //     assert_eq!(output, 0);
    // }
}
