use nom::{
    character::complete::{alpha1, alphanumeric1, char, line_ending},
    combinator::{map, opt},
    multi::{fold_many0, many0},
    sequence::{preceded, separated_pair, terminated},
    IResult,
};
use nom_supreme::tag::complete::tag;
use std::collections::BTreeMap;

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

    let starting_nodes: Vec<&str> = graph
        .keys()
        .filter(|key| key.ends_with("A"))
        .cloned()
        .collect();

    // the cycles are "magically" the lengths from the starting nodes to the end nodes
    // due to how the input for the problem is constructed
    let lengths_to_end_nodes = starting_nodes
        .iter()
        .map(|node| {
            // let mut visited_nodes = vec![*node];
            let mut current_node = *node;

            directions
                .iter()
                .cycle()
                .enumerate()
                .find_map(|(index, dir)| {
                    if let Some((left, right)) = graph.get(current_node) {
                        let next_node = match dir {
                            Direction::Left => left,
                            Direction::Right => right,
                        };

                        // visited_nodes.push(next_node);
                        if next_node.ends_with("Z") {
                            Some(index + 1)
                        } else {
                            current_node = next_node;
                            None
                        }
                    } else {
                        panic!("node {} not found in graph", current_node)
                    }
                })
                .expect("should have found a cycle")
        })
        .collect::<Vec<usize>>();

    // once we know the lenghts of the cycles, we can calculate the lcm of them
    lcm(&lengths_to_end_nodes) as u64
}

fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
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
mod day_08_part2 {
    use super::*;

    #[test]
    fn example1() {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        let output = process(input);
        assert_eq!(output, 6);
    }

    #[test]
    fn input1() {
        let input = include_str!("../../inputs/input1.txt");
        let output = process(input);
        assert_eq!(output, 10371555451871);
    }
}
