use std::vec;

use nom::{
    character::complete::{self, line_ending, space1},
    combinator::{map, opt},
    multi::{many0, separated_list1},
    sequence::terminated,
    IResult,
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Reading {
    values: Vec<i64>,
}

impl Reading {
    fn new(values: Vec<i64>) -> Self {
        Reading { values }
    }

    fn get_interpolations(&self) -> Vec<Vec<i64>> {
        let mut interpolations: Vec<Vec<i64>> = vec![];

        let mut current = self
            .values
            .windows(2)
            .map(|w| {
                let (a, b) = (w[0], w[1]);
                b - a
            })
            .collect::<Vec<i64>>();

        loop {
            interpolations.push(current.clone());
            current = current
                .windows(2)
                .map(|w| {
                    let (a, b) = (w[0], w[1]);
                    b - a
                })
                .collect::<Vec<i64>>();

            if current.iter().all(|n| n == &0) {
                current.push(0);
                interpolations.push(current.clone());
                break;
            }
        }
        interpolations
    }

    fn interpolate_next(&mut self) -> i64 {
        let mut interpolations = self.get_interpolations();
        interpolations.reverse();

        let mut prev: i64 = 0;
        for (idx, interpolation) in interpolations.iter_mut().enumerate() {
            match idx {
                0 => {
                    interpolation.push(prev);
                }
                _ => {
                    prev = prev + interpolation.last().unwrap();
                    interpolation.push(prev);
                }
            }
        }

        let next = prev + self.values.last().unwrap();
        self.values.push(next);
        next
    }
}

fn main() {
    let input = include_str!("../../inputs/input1.txt");
    let output = process(input);
    println!("Output is {output}");
}

fn process(input: &str) -> i64 {
    let (_, mut readings) = parse_input(input).expect("a valid parse");
    readings
        .iter_mut()
        .map(|r| r.interpolate_next())
        .sum::<i64>()
}

fn parse_input(input: &str) -> IResult<&str, Vec<Reading>> {
    many0(terminated(
        map(separated_list1(space1, complete::i64), Reading::new),
        opt(line_ending),
    ))(input)
}

#[cfg(test)]
mod day_09_part1 {
    use super::*;

    #[test]
    fn example1() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        let output = process(input);
        assert_eq!(output, 114);
    }

    #[test]
    fn input1() {
        let input = include_str!("../../inputs/input1.txt");
        let output = process(input);
        assert_eq!(output, 1637452029);
    }
}
