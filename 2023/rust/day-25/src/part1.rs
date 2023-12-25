use crate::custom_error::AocError;
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, line_ending, space1},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};
use petgraph::prelude::*;
use rustworkx_core::connectivity::stoer_wagner_min_cut;
use std::collections::HashMap;
use tracing::debug;

fn parse_wiring_diagram(input: &str) -> IResult<&str, Vec<(&str, Vec<&str>)>> {
    let (input, output) = separated_list1(
        line_ending,
        separated_pair(alpha1, tag(": "), separated_list1(space1, alpha1)),
    )(input)?;

    Ok((input, output))
}

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<u64, AocError> {
    let (_, wires) = parse_wiring_diagram(input).expect("should parse wiring diagram");
    debug!(?wires);

    let nodes = wires
        .iter()
        .flat_map(|(key, values)| {
            let mut nodes = values.clone();
            nodes.push(key);
            nodes
        })
        .unique()
        .collect::<Vec<_>>();

    let mut graph = UnGraph::<&str, u32>::default();

    let node_idx_map: HashMap<&str, NodeIndex> = nodes
        .iter()
        .map(|node| (*node, graph.add_node(&node)))
        .collect();

    for (key, values) in wires.iter() {
        for node in values {
            graph.add_edge(node_idx_map[key], node_idx_map[node], 1);
        }
    }

    let min: rustworkx_core::Result<Option<(usize, Vec<_>)>> =
        stoer_wagner_min_cut(&graph, |_| Ok(1));
    let (_cut_size, nodes_in_partition) = min.unwrap().unwrap();
    let total_nodes = graph.node_count();

    Ok(((total_nodes - nodes_in_partition.len()) * nodes_in_partition.len()) as u64)
}

#[cfg(test)]
mod day_25_part1 {
    use super::*;

    #[test_log::test]
    fn example() -> miette::Result<()> {
        let input = "jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr";
        assert_eq!(54, process(input)?);
        Ok(())
    }

    #[test_log::test]
    fn input1() -> miette::Result<()> {
        let input = include_str!("../inputs/input1.txt");
        assert_eq!(533628, process(input)?);
        Ok(())
    }
}
