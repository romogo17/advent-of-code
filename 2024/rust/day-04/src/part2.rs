use std::collections::HashMap;

use crate::custom_error::AocError;
use glam::IVec2;
use tracing::debug;

const DIRECTIONS: [[IVec2; 2]; 4] = [
    // NorthEast
    [IVec2::new(-1, -1), IVec2::new(1, 1)],
    // SouthEast
    [IVec2::new(-1, 1), IVec2::new(1, -1)],
    // SouthWest
    [IVec2::new(1, 1), IVec2::new(-1, -1)],
    // NorthWest
    [IVec2::new(1, -1), IVec2::new(-1, 1)],
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

    // check every position with an A, and for each direction, check if there is a "M_S"
    let mas = ['M', 'S'];
    let result: usize = positions
        .iter()
        .filter(|(_positions, value)| **value == 'A')
        .filter(|(position, _value)| {
            DIRECTIONS
                .iter()
                .map(|direction| {
                    direction
                        .iter()
                        .map(|offset| positions.get(&(*position + offset)))
                        .enumerate()
                        .all(|(index, value)| mas.get(index) == value)
                })
                .filter(|b| *b)
                .count()
                == 2
        })
        .count();

    Ok(result as u64)
}

#[cfg(test)]
mod day_04_part2 {
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
        assert_eq!(9, process(input)?);
        Ok(())
    }

    #[test_log::test]
    fn input() -> miette::Result<()> {
        let input = include_str!("../inputs/input.txt");
        assert_eq!(1890, process(input)?);
        Ok(())
    }
}
