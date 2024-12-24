use std::collections::HashMap;

use itertools::Itertools;
use miette::miette;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, alphanumeric1, line_ending, multispace1, space1},
    combinator::{all_consuming, opt, value},
    multi::separated_list1,
    sequence::{preceded, separated_pair, terminated, tuple},
    IResult, Parser,
};

use petgraph::{dot::Dot, prelude::DiGraphMap};
use tracing::debug;

#[derive(Debug, Clone)]
enum Operation {
    AND,
    OR,
    XOR,
}

#[derive(Debug, Clone)]
struct Gate<'a> {
    inputs: Vec<&'a str>,
    output: &'a str,
    operation: Operation,
}

fn parse_gate(input: &str) -> IResult<&str, Gate> {
    let (input, elements) = tuple((
        terminated(alphanumeric1, space1),
        alt((
            value(Operation::AND, tag("AND")),
            value(Operation::OR, tag("OR")),
            value(Operation::XOR, tag("XOR")),
        )),
        preceded(space1, alphanumeric1),
        preceded(tag(" -> "), alphanumeric1),
    ))(input)?;

    Ok((
        input,
        Gate {
            inputs: vec![elements.0, elements.2],
            output: elements.3,
            operation: elements.1,
        },
    ))
}

fn parse(input: &str) -> IResult<&str, (HashMap<&str, bool>, Vec<Gate>)> {
    let (input, map) = separated_list1(
        line_ending,
        separated_pair(
            alphanumeric1,
            tag(": "),
            complete::u8.map(|v| match v {
                0 => false,
                1 => true,
                _ => unreachable!("unexpected value"),
            }),
        ),
    )(input)?;

    let (input, gates) = preceded(multispace1, separated_list1(line_ending, parse_gate))(input)?;
    let (input, _) = opt(line_ending)(input)?;

    let map = map.into_iter().collect();

    Ok((input, (map, gates)))
}

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<String> {
    let (_input, (map, gates)) =
        all_consuming(parse)(input).map_err(|e| miette!("parse failed {}", e))?;
    debug!(?map, ?gates);

    let bitstring_x = map
        .iter()
        .filter(|(key, _)| key.starts_with('x'))
        .sorted_by(|a, b| b.0.cmp(&a.0))
        .into_iter()
        .map(|(_, value)| (*value as u8).to_string())
        .collect::<String>();
    let output_x = u64::from_str_radix(&bitstring_x, 2).unwrap();
    debug!(?bitstring_x, ?output_x);

    let bitstring_y = map
        .iter()
        .filter(|(key, _)| key.starts_with('y'))
        .sorted_by(|a, b| b.0.cmp(&a.0))
        .into_iter()
        .map(|(_, value)| (*value as u8).to_string())
        .collect::<String>();
    let output_y = u64::from_str_radix(&bitstring_y, 2).unwrap();
    debug!(?bitstring_y, ?output_y);

    debug!(
        "binary_sum={:b}, sum={}",
        output_x + output_y,
        output_x + output_y
    );

    let mut current_map = map.clone();
    let mut pending_gates = gates.clone();
    let mut processed_gates: Vec<Gate> = vec![];

    while !pending_gates.is_empty() {
        let ready_gates = pending_gates
            .extract_if(.., |Gate { inputs, .. }| {
                inputs
                    .iter()
                    .all(|input_key| current_map.contains_key(input_key))
            })
            .collect::<Vec<_>>();

        for gate in ready_gates {
            let a = current_map.get(gate.inputs[0]).unwrap();
            let b = current_map.get(gate.inputs[1]).unwrap();

            let value = match gate.operation {
                Operation::AND => a & b,
                Operation::OR => a | b,
                Operation::XOR => a ^ b,
            };

            current_map.entry(&gate.output).or_insert(value);
            processed_gates.push(gate);
        }
    }

    let node_names = gates
        .iter()
        .map(|gate| {
            (
                gate.output,
                format!(
                    "{}\n{}",
                    gate.output,
                    match gate.operation {
                        Operation::AND => "AND",
                        Operation::OR => "OR",
                        Operation::XOR => "XOR",
                    }
                ),
            )
        })
        .collect::<HashMap<&str, String>>();

    let edges = gates
        .iter()
        .flat_map(|Gate { inputs, output, .. }| {
            inputs
                .iter()
                .map(|input| {
                    (
                        node_names.get(input).map(|v| v.as_str()).unwrap_or(input),
                        node_names.get(output).map(|v| v.as_str()).unwrap_or(output),
                        current_map.get(input).unwrap(),
                    )
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let g = &DiGraphMap::<&str, &bool>::from_edges(&edges);
    println!("{}", Dot::with_config(&g, &[]));

    let bitstring = current_map
        .iter()
        .filter(|(key, _)| key.starts_with("z"))
        .sorted_by(|a, b| b.0.cmp(&a.0))
        .into_iter()
        .map(|(_, value)| (*value as u8).to_string())
        .collect::<String>();

    debug!(?bitstring);

    let result = u64::from_str_radix(&bitstring, 2).unwrap();

    Ok(result.to_string())
}
