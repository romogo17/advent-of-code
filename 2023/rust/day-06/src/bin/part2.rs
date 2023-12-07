use day_06::*;

use nom::{
    bytes::complete::take_until,
    character::complete::{self},
    IResult, Parser,
};
use nom_supreme::{tag::complete::tag, ParserExt};

fn main() {
    let input = include_str!("../../inputs/input1.txt");
    let output = process(input);
    println!("Output is {output}");
}

fn process(input: &str) -> u64 {
    let boat_race = parse(input);
    boat_race.ways_to_win()
}

fn parse_line(input: &str) -> IResult<&str, u64> {
    let (input, val) = take_until(":")
        .precedes(tag(":"))
        .precedes(complete::u64)
        .parse(input)?;

    Ok((input, val))
}

fn parse(input: &str) -> BoatRace {
    let parsed = input
        .lines()
        .map(|line| {
            let (_, v) = parse_line(line.replace(" ", "").as_str()).expect("a valid line");
            v
        })
        .collect::<Vec<u64>>();

    BoatRace::new(parsed[0], parsed[1])
}

#[cfg(test)]
mod day_06_part2 {
    use super::*;

    #[test]
    fn example() {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        let output = process(input);
        assert_eq!(output, 71503);
    }

    // #[test]
    // fn input1() {
    //     let input = include_str!("../../inputs/input1.txt");
    //     let output = process(input);
    //     assert_eq!(output, 5133600);
    // }
}
