use day_11::*;

use glam::IVec2;
use itertools::Itertools;
use nom::{
    branch::alt, bytes::complete::tag, character::complete::multispace0, combinator::all_consuming,
    multi::many1, sequence::terminated, IResult, Parser,
};
use std::collections::HashMap;

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

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

fn expand_space(input: &str) -> String {
    let rows = input.lines().count();
    let cols = input.lines().next().unwrap().chars().count();

    let (_, space) = parse_space_grid(Span::new(input)).expect("should parse a space grid");

    let empty_rows = (0..rows)
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

    let mut space = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    for row in empty_rows.iter().rev() {
        space.insert(*row, vec!['.'; cols])
    }
    space = transpose(space);

    for col in empty_cols.iter().rev() {
        space.insert(*col, vec!['.'; rows + empty_rows.len()])
    }
    space = transpose(space);

    space
        .iter()
        .map(|row| row.into_iter().collect::<String>())
        .collect::<Vec<String>>()
        .join("\n")
}

fn process(input: &str) -> u64 {
    let expanded_space = expand_space(input);

    let (_, space) =
        parse_space_grid(Span::new(expanded_space.as_str())).expect("should parse a space grid");

    let galaxies = space
        .iter()
        .filter(|(_, space_type)| **space_type == SpaceType::Galaxy)
        .map(|(location, _)| *location)
        .collect::<Vec<_>>();

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
mod day_11_part1 {
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
        assert_eq!(output, 374);
    }

    // #[test]
    // fn input1() {
    //     let input = include_str!("../../inputs/input1.txt");
    //     let output = process(input);
    //     assert_eq!(output, 6856);
    // }
}
