use day_09::*;

use nom::{
    character::complete::{self, line_ending, space1},
    combinator::{map, opt},
    multi::{many0, separated_list1},
    sequence::terminated,
    IResult,
};

fn main() {
    let input = include_str!("../../inputs/input1.txt");
    let output = process(input);
    println!("Output is {output}");
}

fn process(input: &str) -> i64 {
    let (_, mut readings) = parse_input(input).expect("a valid parse");
    readings
        .iter_mut()
        .map(|r| r.interpolate_prev())
        .sum::<i64>()
}

fn parse_input(input: &str) -> IResult<&str, Vec<Reading>> {
    many0(terminated(
        map(separated_list1(space1, complete::i64), Reading::new),
        opt(line_ending),
    ))(input)
}

#[cfg(test)]
mod day_09_part2 {
    use super::*;

    #[test]
    fn example() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        let output = process(input);
        assert_eq!(output, 2);
    }

    #[test]
    fn input1() {
        let input = include_str!("../../inputs/input1.txt");
        let output = process(input);
        assert_eq!(output, 908);
    }
}
