use std::collections::BTreeMap;

fn move_rocks_north(input: &str) -> Vec<Vec<char>> {
    // Transpose the rows into columns
    let mut columns_iter_collection = input.lines().map(|line| line.chars()).collect::<Vec<_>>();
    let colums = std::iter::from_fn(move || {
        let mut items = vec![];
        for iter in &mut columns_iter_collection {
            match iter.next() {
                Some(item) => items.push(item),
                None => return None,
            }
        }
        Some(items)
    })
    .collect::<Vec<Vec<char>>>();

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

fn process(input: &str) -> u64 {
    let moved_rocks = move_rocks_north(input);
    compute_load_north_beam(moved_rocks)
}

fn main() {
    let input = include_str!("../../inputs/input1.txt");
    let output = process(input);
    println!("Output is {output}");
}

#[cfg(test)]
mod day_14_part1 {
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
        assert_eq!(output, 136);
    }

    #[test]
    fn input1() {
        let input = include_str!("../../inputs/input1.txt");
        let output = process(input);
        assert_eq!(output, 108935);
    }
}
