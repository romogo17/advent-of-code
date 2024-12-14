use std::collections::HashMap;

use petgraph::{algo::condensation, prelude::*, visit::IntoNodeReferences};

const DIRECTIONS: [[i32; 2]; 4] = [[0, 1], [1, 0], [0, -1], [-1, 0]];

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<u32> {
    let map = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, c)| ((x as i32, y as i32), c))
        })
        .collect::<HashMap<(i32, i32), char>>();

    let mut graph: UnGraphMap<(i32, i32), ()> = UnGraphMap::new();

    // build and connect the graph
    for ((x, y), c) in map.iter() {
        let node = graph.add_node((*x, *y));

        for [x1, y1] in DIRECTIONS.iter() {
            let adjacent_node = (x + x1, y + y1);
            if map.get(&adjacent_node).is_some_and(|c1| c == c1) {
                graph.add_edge(node, adjacent_node, ());
            };
        }
    }

    // condense every strongly connected component into a single node
    let condensed_graph = condensation(graph.clone().into_graph::<NodeIndex>(), false);
    let result = condensed_graph
        .node_references()
        .map(|(_node_idx, node_list)| {
            let area = node_list.len();
            let perimeter = node_list
                .iter()
                .map(|n| 4 - graph.neighbors(*n).count())
                .sum::<usize>();
            area * perimeter
        })
        .sum::<usize>();

    Ok(result as u32)
}

#[cfg(test)]
mod day_12_part1 {
    use super::*;

    #[test_log::test]
    fn example() -> miette::Result<()> {
        let input = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";
        assert_eq!(1930, process(input)?);
        Ok(())
    }

    #[test_log::test]
    fn input() -> miette::Result<()> {
        let input = include_str!("../inputs/input.txt");
        assert_eq!(1377008, process(input)?);
        Ok(())
    }
}
