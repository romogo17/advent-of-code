use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, HashMap};

use itertools::Itertools;

const TOTAL_CYCLES: u64 = 1_000_000_000;

fn move_rocks_north(rocks: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    // Transpose the rows into columns
    let mut columns_iter_collection = rocks.iter().map(|row| row.iter()).collect::<Vec<_>>();
    let colums = std::iter::from_fn(move || {
        let mut items = vec![];
        for iter in &mut columns_iter_collection {
            match iter.next() {
                Some(item) => items.push(item.clone()),
                None => return None,
            }
        }
        Some(items)
    })
    .collect::<Vec<Vec<char>>>();

    // Move the rocks
    let colums: Vec<Vec<char>> = colums
        .iter()
        .map(|col| {
            let mut intervals: BTreeMap<Option<usize>, usize> = BTreeMap::new();
            let mut interval_start = None;
            intervals.insert(interval_start, 0);

            for (idx, rock) in col.iter().enumerate() {
                match rock {
                    '#' => {
                        interval_start = Some(idx);
                        intervals.insert(interval_start, 0);
                    }
                    'O' => {
                        intervals.entry(interval_start).and_modify(|val| *val += 1);
                    }
                    '.' => {}
                    _ => unreachable!(),
                }
            }
            (col.len(), intervals)
        })
        .map(|(col_len, intervals)| {
            let mut result = vec![];
            let mut idx = 0;
            for (start, rock_count) in intervals {
                match start {
                    None => {
                        result.append(vec!['O'; rock_count].as_mut());
                        idx = rock_count;
                    }
                    Some(start) => {
                        result.append(vec!['.'; start - idx].as_mut());
                        result.push('#');
                        result.append(vec!['O'; rock_count].as_mut());
                        idx = start + rock_count + 1;
                    }
                }
            }
            result.append(vec!['.'; col_len - idx].as_mut());
            result
        })
        .collect();

    // Transpose the columns into rows
    let mut rows_iter_collection = colums.iter().map(|col| col.iter()).collect::<Vec<_>>();
    let rows = std::iter::from_fn(move || {
        let mut items = vec![];
        for iter in &mut rows_iter_collection {
            match iter.next() {
                Some(item) => items.push(item.clone()),
                None => return None,
            }
        }
        Some(items)
    })
    .collect::<Vec<Vec<char>>>();

    rows
}

fn move_rocks_south(rocks: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    // Transpose the rows into reversed columns
    let mut columns_iter_collection = rocks
        .iter()
        .map(|row| row.iter().rev())
        .rev()
        .collect::<Vec<_>>();
    let colums = std::iter::from_fn(move || {
        let mut items = vec![];
        for iter in &mut columns_iter_collection {
            match iter.next() {
                Some(item) => items.push(item.clone()),
                None => return None,
            }
        }
        Some(items)
    })
    .collect::<Vec<Vec<char>>>();

    // Move the rocks
    let colums: Vec<Vec<char>> = colums
        .iter()
        .map(|col| {
            let mut intervals: BTreeMap<Option<usize>, usize> = BTreeMap::new();
            let mut interval_start = None;
            intervals.insert(interval_start, 0);

            for (idx, rock) in col.iter().enumerate() {
                match rock {
                    '#' => {
                        interval_start = Some(idx);
                        intervals.insert(interval_start, 0);
                    }
                    'O' => {
                        intervals.entry(interval_start).and_modify(|val| *val += 1);
                    }
                    '.' => {}
                    _ => unreachable!(),
                }
            }
            (col.len(), intervals)
        })
        .map(|(col_len, intervals)| {
            let mut result = vec![];
            let mut idx = 0;
            for (start, rock_count) in intervals {
                match start {
                    None => {
                        result.append(vec!['O'; rock_count].as_mut());
                        idx = rock_count;
                    }
                    Some(start) => {
                        result.append(vec!['.'; start - idx].as_mut());
                        result.push('#');
                        result.append(vec!['O'; rock_count].as_mut());
                        idx = start + rock_count + 1;
                    }
                }
            }
            result.append(vec!['.'; col_len - idx].as_mut());
            result
        })
        .collect();

    // Transpose the columns into reversed rows
    let mut rows_iter_collection = colums
        .iter()
        .map(|col| col.iter().rev())
        .rev()
        .collect::<Vec<_>>();
    let rows = std::iter::from_fn(move || {
        let mut items = vec![];
        for iter in &mut rows_iter_collection {
            match iter.next() {
                Some(item) => items.push(item.clone()),
                None => return None,
            }
        }
        Some(items)
    })
    .collect::<Vec<Vec<char>>>();

    rows
}

fn move_rocks_west(rocks: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    // Clone the rocks
    let rows = rocks
        .iter()
        .map(|row| row.iter().cloned().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    // Move the rocks
    let rows: Vec<Vec<char>> = rows
        .iter()
        .map(|col| {
            let mut intervals: BTreeMap<Option<usize>, usize> = BTreeMap::new();
            let mut interval_start = None;
            intervals.insert(interval_start, 0);

            for (idx, rock) in col.iter().enumerate() {
                match rock {
                    '#' => {
                        interval_start = Some(idx);
                        intervals.insert(interval_start, 0);
                    }
                    'O' => {
                        intervals.entry(interval_start).and_modify(|val| *val += 1);
                    }
                    '.' => {}
                    _ => unreachable!(),
                }
            }
            (col.len(), intervals)
        })
        .map(|(col_len, intervals)| {
            let mut result = vec![];
            let mut idx = 0;
            for (start, rock_count) in intervals {
                match start {
                    None => {
                        result.append(vec!['O'; rock_count].as_mut());
                        idx = rock_count;
                    }
                    Some(start) => {
                        result.append(vec!['.'; start - idx].as_mut());
                        result.push('#');
                        result.append(vec!['O'; rock_count].as_mut());
                        idx = start + rock_count + 1;
                    }
                }
            }
            result.append(vec!['.'; col_len - idx].as_mut());
            result
        })
        .collect();

    // Clone the rocks
    rows.iter()
        .map(|row| row.iter().cloned().collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

fn move_rocks_east(rocks: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    // Reverse the rows
    let rows = rocks
        .iter()
        .map(|row| row.iter().cloned().rev().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    // Move the rocks
    let rows: Vec<Vec<char>> = rows
        .iter()
        .map(|col| {
            let mut intervals: BTreeMap<Option<usize>, usize> = BTreeMap::new();
            let mut interval_start = None;
            intervals.insert(interval_start, 0);

            for (idx, rock) in col.iter().enumerate() {
                match rock {
                    '#' => {
                        interval_start = Some(idx);
                        intervals.insert(interval_start, 0);
                    }
                    'O' => {
                        intervals.entry(interval_start).and_modify(|val| *val += 1);
                    }
                    '.' => {}
                    _ => unreachable!(),
                }
            }
            (col.len(), intervals)
        })
        .map(|(col_len, intervals)| {
            let mut result = vec![];
            let mut idx = 0;
            for (start, rock_count) in intervals {
                match start {
                    None => {
                        result.append(vec!['O'; rock_count].as_mut());
                        idx = rock_count;
                    }
                    Some(start) => {
                        result.append(vec!['.'; start - idx].as_mut());
                        result.push('#');
                        result.append(vec!['O'; rock_count].as_mut());
                        idx = start + rock_count + 1;
                    }
                }
            }
            result.append(vec!['.'; col_len - idx].as_mut());
            result
        })
        .collect();

    // Reverse the rows
    rows.iter()
        .map(|row| row.iter().cloned().rev().collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

fn compute_load_north_beam(rocks: Vec<Vec<char>>) -> u64 {
    let row_count = rocks.len();
    rocks
        .iter()
        .enumerate()
        .map(|(idx, row)| {
            (row.iter().filter(|rock| *rock == &'O').count() * (row_count - idx)) as u64
        })
        .sum()
}

fn hash_rocks(rocks: &Vec<Vec<char>>) -> String {
    let mut hasher = Sha256::new();
    hasher.update(
        rocks
            .iter()
            .map(|row| row.iter().join(""))
            .join("")
            .as_bytes(),
    );
    let sha256digest = hasher.finalize();
    format!("{:x}", sha256digest)
}

fn process(input: &str) -> u64 {
    let mut rocks = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut cache = HashMap::new();
    let mut end_cycle = None;

    for cycle in 0..TOTAL_CYCLES {
        rocks = move_rocks_north(&rocks);
        rocks = move_rocks_west(&rocks);
        rocks = move_rocks_south(&rocks);
        rocks = move_rocks_east(&rocks);

        let hash = hash_rocks(&rocks);
        match cache.get(&hash) {
            Some(past_cycle) => {
                let period = cycle - past_cycle;

                if end_cycle.is_none() {
                    let extra_cycles =
                        (TOTAL_CYCLES - past_cycle) - (TOTAL_CYCLES - past_cycle) / period * period;
                    end_cycle = Some(cycle + extra_cycles - 1);
                }
            }
            None => {
                cache.insert(hash, cycle);
            }
        }
        if end_cycle.is_some_and(|end_cycle| cycle == end_cycle) {
            break;
        }
    }
    compute_load_north_beam(rocks)
}

fn main() {
    let input = include_str!("../../inputs/input1.txt");
    let output = process(input);
    println!("Output is {output}");
}

#[cfg(test)]
mod day_14_part2 {
    use super::*;

    #[test]
    fn example() {
        let input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
        let output = process(input);
        assert_eq!(output, 64);
    }

    #[test]
    fn input1() {
        let input = include_str!("../../inputs/input1.txt");
        let output = process(input);
        assert_eq!(output, 100876);
    }
}
