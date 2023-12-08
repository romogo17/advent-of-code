use petgraph::graphmap::{DiGraphMap, GraphMap};
use std::collections::HashMap;

use nom::{
    bytes::complete::{take_till, take_until},
    character::{
        complete::{self, alpha1, alphanumeric1, char, line_ending, space1},
        is_alphabetic,
    },
    combinator::{map, opt},
    multi::{fold_many0, many0},
    sequence::{self, delimited, preceded, separated_pair, terminated},
    IResult, Parser,
};
use nom_supreme::{tag::complete::tag, ParserExt};

#[derive(Debug)]
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
    // let input = include_str!("../../inputs/input1.txt");
    let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
    let output = process(input);
    println!("Output is {output}");
}

fn process(input: &str) -> u64 {
    let (directions, graph) = parse_input(input);

    dbg!(directions);
    dbg!(graph);
    100
}

// fn parse_input(input: &str) -> (Vec<Direction>, DiGraphMap<&str, i32>) {
fn parse_input(input: &str) -> (Vec<Direction>, HashMap<&str, (&str, &str)>) {
    let (remaining, directions) = parse_directions(input).unwrap();
    let (_, graph) = preceded(many0(line_ending), parse_graph)(remaining).unwrap();

    (directions, graph)
}

fn parse_directions(input: &str) -> IResult<&str, Vec<Direction>> {
    map(alpha1, |directions: &str| {
        directions.chars().map(Direction::new).collect()
    })(input)
}

// fn parse_graph(input: &str) -> IResult<&str, DiGraphMap<&str, i32>> {
fn parse_graph(input: &str) -> IResult<&str, HashMap<&str, (&str, &str)>> {
    // This would return a Vec<(&str, (&str, &str))>:
    // many0(parse_graph_line)(input)

    fold_many0(
        parse_graph_line,
        // || DiGraphMap::new(),
        || HashMap::new(),
        // |mut map, (key, (left, right))| {
        |mut map, (key, value)| {
            // map.add_edge(key, left, -1);
            // map.add_edge(key, right, -1);
            map.insert(key, value);
            map
        },
    )(input)
}

fn parse_graph_line(input: &str) -> IResult<&str, (&str, (&str, &str))> {
    terminated(
        separated_pair(
            alpha1,
            tag(" = "),
            separated_pair(
                preceded(char('('), alpha1),
                tag(", "),
                terminated(alpha1, char(')')),
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
        assert_eq!(output, 246912307);
    }
}
