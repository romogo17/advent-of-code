use day_06::*;

use nom::{
    bytes::complete::take_until,
    character::complete::{self, space1},
    multi::separated_list1,
    IResult, Parser,
};
use nom_supreme::{tag::complete::tag, ParserExt};

fn main() {
    let input = include_str!("../../inputs/input1.txt");
    let output = process(input);
    println!("Output is {output}");
}

fn process(input: &str) -> u64 {
    let boat_races = parse(input);
    boat_races.iter().map(|br| br.naive_ways_to_win()).product()
}

fn parse_line(input: &str) -> IResult<&str, Vec<u64>> {
    let (input, vals) = take_until(":")
        .precedes(tag(":"))
        .precedes(space1)
        .precedes(separated_list1(space1, complete::u64))
        .parse(input)?;

    Ok((input, vals))
}

fn parse(input: &str) -> Vec<BoatRace> {
    let parsed = input
        .lines()
        .map(|line| {
            let (_, v) = parse_line(line).expect("a valid line");
            v
        })
        .collect::<Vec<Vec<u64>>>();

    parsed[0]
        .iter()
        .zip(parsed[1].iter())
        .map(|(t, d)| BoatRace::new(*t, *d))
        .collect()
}

#[cfg(test)]
mod day_06_part1 {
    use super::*;

    #[test]
    fn example() {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        let output = process(input);
        assert_eq!(output, 288);
    }

    #[test]
    fn input1() {
        let input = include_str!("../../inputs/input1.txt");
        let output = process(input);
        assert_eq!(output, 5133600);
    }
}
