use day_10::*;

use glam::IVec2;
use nom::{
    branch::alt, bytes::complete::tag, character::complete::multispace0, combinator::all_consuming,
    multi::many1, sequence::terminated, IResult, Parser,
};
use std::collections::{HashMap, HashSet};

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

fn find_cycle(grid: &HashMap<IVec2, PipeType>, start_position: &IVec2) -> HashSet<IVec2> {
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
    let mut visited = HashSet::new();

    loop {
        visited.insert(current_position);

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

    // Implements a 2d version of the ray casting algorithm to figure out if a particular position is
    // inside of the cycle defined by `cycle`.
    //
    // One simple way of finding whether the point is inside or outside a simple polygon is to test
    // how many times a ray, starting from the point and going in any fixed direction, intersects the
    // edges of the polygon. If the point is on the outside of the polygon the ray will intersect its
    // edge an even number of times. If the point is on the inside of the polygon then it will
    // intersect the edge an odd number of times.
    //
    // https://en.wikipedia.org/wiki/Point_in_polygon#Ray_casting_algorithm
    let positions_in_cycle: u64 = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            let mut line_score: u64 = 0;
            let mut crossings: u64 = 0;

            for c in line.chars().enumerate().map(|(x, c)| {
                match cycle.contains(&IVec2::new(x as i32, y as i32)) {
                    true => c,
                    false => '.',
                }
            }) {
                match c {
                    '.' => {
                        if crossings % 2 != 0 {
                            line_score += 1;
                        }
                    }
                    'S' => {
                        crossings += 1;
                    }
                    '|' => {
                        crossings += 1;
                    }
                    'F' => {
                        crossings += 1;
                    }
                    '7' => {
                        crossings += 1;
                    }
                    'L' => {}
                    'J' => {}
                    '-' => {}
                    value => unreachable!("unexpected value: {}", value),
                }
            }
            line_score
        })
        .sum();

    positions_in_cycle
}

fn main() {
    let input = include_str!("../../inputs/input1.txt");
    let output = process(input);
    println!("Output is {output}");
}

#[cfg(test)]
mod day_10_part2 {
    use super::*;

    #[test]
    fn example1() {
        let input = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";
        let output = process(input);
        assert_eq!(output, 4);
    }

    #[test]
    fn example2() {
        let input = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";
        let output = process(input);
        assert_eq!(output, 8);
    }

    #[test]
    fn example3() {
        let input = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";
        let output = process(input);
        assert_eq!(output, 10);
    }

    #[test]
    fn input1() {
        let input = include_str!("../../inputs/input1.txt");
        let output = process(input);
        assert_eq!(output, 501);
    }
}
