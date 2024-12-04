use std::collections::HashMap;

use crate::custom_error::AocError;
use glam::IVec2;
use tracing::{debug, info};

const DIRECTIONS: [[IVec2; 3]; 8] = [
    // North
    [IVec2::new(0, 1), IVec2::new(0, 2), IVec2::new(0, 3)],
    // South
    [IVec2::new(0, -1), IVec2::new(0, -2), IVec2::new(0, -3)],
    // NorthEast
    [IVec2::new(1, 1), IVec2::new(2, 2), IVec2::new(3, 3)],
    // SouthEast
    [IVec2::new(1, -1), IVec2::new(2, -2), IVec2::new(3, -3)],
    // NorthWest
    [IVec2::new(-1, 1), IVec2::new(-2, 2), IVec2::new(-3, 3)],
    // SouthWest
    [IVec2::new(-1, -1), IVec2::new(-2, -2), IVec2::new(-3, -3)],
    // East
    [IVec2::new(1, 0), IVec2::new(2, 0), IVec2::new(3, 0)],
    // West
    [IVec2::new(-1, 0), IVec2::new(-2, 0), IVec2::new(-3, 0)],
];

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<u64, AocError> {
    debug!("input: \n{}", input);

    let positions = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, c)| (IVec2::new(x as i32, y as i32), c))
        })
        .collect::<HashMap<IVec2, char>>();

    // check every position with an X, and for each direction, check if there is a "MAS"
    let mas = ['M', 'A', 'S'];
    let result: usize = positions
        .iter()
        .filter(|(_positions, value)| **value == 'X')
        .map(|(position, value)| {
            let matches = DIRECTIONS
                .iter()
                .map(|direction| {
                    direction
                        .iter()
                        .map(|offset| positions.get(&(position + offset)))
                        .enumerate()
                        .all(|(index, value)| mas.get(index) == value)
                })
                .filter(|b| *b)
                .count();
            info!(?position, ?value, ?matches);
            matches
        })
        .sum();

    Ok(result as u64)
}

#[cfg(test)]
mod day_04_part1 {
    use super::*;

    #[test_log::test]
    fn example() -> miette::Result<()> {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        assert_eq!(18, process(input)?);
        Ok(())
    }

    #[test_log::test]
    fn input() -> miette::Result<()> {
        let input = include_str!("../inputs/input.txt");
        assert_eq!(2493, process(input)?);
        Ok(())
    }
}
