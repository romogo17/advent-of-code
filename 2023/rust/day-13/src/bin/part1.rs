// use day_13::*;

use itertools::Itertools;

enum Reflection {
    Horizontal(usize),
    Vertical(usize),
}

fn find_vertical_reflection(input: &str) -> Option<Reflection> {
    // This iterates over colums of a 2D array.
    //
    // First, we create a vec of iterators over each line.
    // Then, we yield 1 item from each iterator, which is a column.
    // Doing this until we run out of items in the iterators transposes the 2D array.
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

    let result = colums
        .iter()
        .enumerate()
        .tuple_windows()
        .filter(|((_, line_a), (_, line_b))| line_a == line_b)
        .find_map(|((index_a, _), (index_b, _))| {
            let cols_a = (&colums[0..=index_a]).iter().rev();
            let cols_b = (&colums[index_b..]).iter();

            cols_a
                .zip(cols_b)
                .all(|(a, b)| a == b)
                .then_some(index_a + 1)
        });

    result.map(|num| Reflection::Vertical(num))
}

fn find_horizontal_reflection(input: &str) -> Option<Reflection> {
    let rows: Vec<&str> = input.lines().collect();
    let result = input
        .lines()
        .enumerate()
        .tuple_windows()
        .filter(|((_, line_a), (_, line_b))| line_a == line_b)
        .find_map(|((index_a, _), (index_b, _))| {
            let rows_a = (&rows[0..=index_a]).iter().rev();
            let rows_b = (&rows[index_b..]).iter();

            rows_a
                .zip(rows_b)
                .all(|(a, b)| a == b)
                .then_some(index_a + 1)
        });

    result.map(|num| Reflection::Horizontal(num))
}

fn find_reflection(input: &str) -> Option<Reflection> {
    find_vertical_reflection(input).or(find_horizontal_reflection(input))
}

fn process(input: &str) -> u64 {
    let (horizontal, vertical) =
        input
            .split("\n\n")
            .flat_map(find_reflection)
            .fold((0usize, 0usize), |mut acc, item| match item {
                Reflection::Horizontal(num) => {
                    acc.0 += 100 * num;
                    acc
                }
                Reflection::Vertical(num) => {
                    acc.1 += num;
                    acc
                }
            });

    (horizontal + vertical) as u64
}

fn main() {
    let input = include_str!("../../inputs/input1.txt");
    let output = process(input);
    println!("Output is {output}");
}

#[cfg(test)]
mod day_13_part1 {
    use super::*;

    #[test]
    fn example() {
        let input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
        let output = process(input);
        assert_eq!(output, 405);
    }

    #[test]
    fn input1() {
        let input = include_str!("../../inputs/input1.txt");
        let output = process(input);
        assert_eq!(output, 37113);
    }
}
