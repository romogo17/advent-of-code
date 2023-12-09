use std::collections::BTreeMap;

use nom::{
    character::complete::{alpha1, alphanumeric1, char, line_ending},
    combinator::{map, opt},
    multi::{fold_many0, many0},
    sequence::{preceded, separated_pair, terminated},
    IResult,
};
use nom_supreme::tag::complete::tag;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    Left,
    Right,
}

impl Direction {
    fn new(input: char) -> Self {
        match input {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => unreachable!(),
        }
    }
}

fn main() {
    let input = include_str!("../../inputs/input1.txt");
    let output = process(input);
    println!("Output is {output}");
}

fn process(input: &str) -> u64 {
    let (directions, graph) = parse_input(input);

    let mut dir_iter = directions.iter().cycle();
    let mut node = "AAA";
    let mut visits = 0;

    while let Some(dir) = dir_iter.next() {
        if let Some((left, right)) = graph.get(node) {
            node = match dir {
                Direction::Left => left,
                Direction::Right => right,
            };
            visits += 1;

            if node == "ZZZ" {
                return visits;
            }
        }
    }

    visits
}

fn parse_input(input: &str) -> (Vec<Direction>, BTreeMap<&str, (&str, &str)>) {
    let (remaining, directions) = parse_directions(input).unwrap();
    let (_, graph) = preceded(many0(line_ending), parse_graph)(remaining).unwrap();

    (directions, graph)
}

fn parse_directions(input: &str) -> IResult<&str, Vec<Direction>> {
    map(alpha1, |directions: &str| {
        directions.chars().map(Direction::new).collect()
    })(input)
}

fn parse_graph(input: &str) -> IResult<&str, BTreeMap<&str, (&str, &str)>> {
    fold_many0(
        parse_graph_line,
        || BTreeMap::new(),
        |mut map, (key, value)| {
            map.insert(key, value);
            map
        },
    )(input)
}

fn parse_graph_line(input: &str) -> IResult<&str, (&str, (&str, &str))> {
    terminated(
        separated_pair(
            alphanumeric1,
            tag(" = "),
            separated_pair(
                preceded(char('('), alphanumeric1),
                tag(", "),
                terminated(alphanumeric1, char(')')),
            ),
        ),
        opt(line_ending),
    )(input)
}

#[cfg(test)]
mod day_08_part1 {
    use super::*;

    #[test]
    fn example1() {
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
        let output = process(input);
        assert_eq!(output, 2);
    }

    #[test]
    fn example2() {
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        let output = process(input);
        assert_eq!(output, 6);
    }

    #[test]
    fn input1() {
        let input = include_str!("../../inputs/input1.txt");
        let output = process(input);
        assert_eq!(output, 22357);
    }
}
