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

    let bitstring = current_map
        .iter()
        .filter(|(key, _)| key.starts_with('z'))
        .sorted_by(|a, b| b.0.cmp(&a.0))
        .into_iter()
        .map(|(_, value)| (*value as u8).to_string())
        .collect::<String>();

    let result = u64::from_str_radix(&bitstring, 2).unwrap();

    Ok(result.to_string())
}

#[cfg(test)]
mod day_24_part1 {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(
        "x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02",
        "4"
    )]
    #[case(
        "x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj",
        "2024"
    )]
    #[test_log::test]
    fn example(#[case] input: &str, #[case] output: &str) -> miette::Result<()> {
        assert_eq!(output, process(input)?);
        Ok(())
    }

    #[test_log::test]
    fn input() -> miette::Result<()> {
        let input = include_str!("../inputs/input.txt");
        assert_eq!("51837135476040", process(input)?);
        Ok(())
    }
}
