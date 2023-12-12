use day_11::*;

use glam::IVec2;
use itertools::Itertools;
use nom::{
    branch::alt, bytes::complete::tag, character::complete::multispace0, combinator::all_consuming,
    multi::many1, sequence::terminated, IResult, Parser,
};
use std::collections::HashMap;

const SPACE_EXPANSION_RATE: u64 = 1_000_000;

fn parse_space_grid(input: Span) -> IResult<Span, HashMap<IVec2, SpaceType>> {
    let (input, output) = all_consuming(many1(terminated(
        alt((
            tag("#").map(with_xy).map(|span| SpaceInfo {
                span,
                space_type: SpaceType::Galaxy,
            }),
            tag(".").map(with_xy).map(|span| SpaceInfo {
                span,
                space_type: SpaceType::Empty,
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

fn get_empty_rows_cols(
    rows: usize,
    cols: usize,
    space: &HashMap<IVec2, SpaceType>,
) -> (Vec<usize>, Vec<usize>) {
    let empty_rows: Vec<usize> = (0..rows)
        .filter_map(|y| {
            let is_row_empty = (0..cols)
                .filter_map(|x| space.get(&IVec2::new(x as i32, y as i32)))
                .all(|space_type| space_type == &SpaceType::Empty);

            match is_row_empty {
                true => Some(y),
                false => None,
            }
        })
        .collect::<Vec<_>>();

    let empty_cols: Vec<usize> = (0..cols)
        .filter_map(|x| {
            let is_col_empty = (0..rows)
                .filter_map(|y| space.get(&IVec2::new(x as i32, y as i32)))
                .all(|space_type| space_type == &SpaceType::Empty);
            match is_col_empty {
                true => Some(x),
                false => None,
            }
        })
        .collect::<Vec<_>>();

    (empty_rows, empty_cols)
}

fn process(input: &str) -> u64 {
    let rows = input.lines().count();
    let cols = input.lines().next().unwrap().chars().count();

    let (_, space) = parse_space_grid(Span::new(input)).expect("should parse a space grid");
    let (mut empty_rows, mut empty_cols) = get_empty_rows_cols(rows, cols, &space);

    // Get the galaxies
    let mut galaxies = space
        .iter()
        .filter(|(_, space_type)| **space_type == SpaceType::Galaxy)
        .map(|(location, _)| *location)
        .collect::<Vec<_>>();

    // Expand the indices of the empty rows and cols
    // We substract 1 because we're replacing the empty row/col with SPACE_EXPANSION_RATE,
    // not adding it
    empty_rows = empty_rows
        .iter()
        .enumerate()
        .map(|(idx, row)| row + (idx * SPACE_EXPANSION_RATE as usize) - (idx * 1))
        .collect();

    empty_cols = empty_cols
        .iter()
        .enumerate()
        .map(|(idx, col)| col + (idx * SPACE_EXPANSION_RATE as usize) - (idx * 1))
        .collect();

    // Expand the galaxies
    for (_idx, row) in empty_rows.iter().enumerate() {
        for galaxy in galaxies.iter_mut() {
            if galaxy.y > *row as i32 {
                galaxy.y += SPACE_EXPANSION_RATE as i32 - 1;
            }
        }
    }

    for (_idx, col) in empty_cols.iter().enumerate() {
        for galaxy in galaxies.iter_mut() {
            if galaxy.x > *col as i32 {
                galaxy.x += SPACE_EXPANSION_RATE as i32 - 1;
            }
        }
    }

    galaxies
        .iter()
        .combinations(2)
        .map(|pair| {
            // This is the Manhathan distance, where the distance between two
            // points is the sum of the absolute differences of their Cartesian coordinates.
            // https://en.wikipedia.org/wiki/Taxicab_geometry
            let (a, b) = (pair[0], pair[1]);
            let distance = (a.x - b.x).abs() + (a.y - b.y).abs();
            distance as u64
        })
        .sum()
}

fn main() {
    let input = include_str!("../../inputs/input1.txt");
    let output = process(input);
    println!("Output is {output}");
}

#[cfg(test)]
mod day_11_part2 {
    use super::*;

    #[test]
    fn example() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        let output = process(input);
        assert_eq!(output, 82000210);
    }

    #[test]
    fn input1() {
        let input = include_str!("../../inputs/input1.txt");
        let output = process(input);
        assert_eq!(output, 603020563700);
    }
}
