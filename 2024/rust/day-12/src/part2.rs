use std::collections::HashMap;

use itertools::Itertools;
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
            let group_id = map.get(&node_list[0]).unwrap();
            let area = node_list.len();
            let sides = node_list
                .iter()
                .map(|n| corner_count(n, &map, group_id))
                .sum::<usize>();
            area * sides
        })
        .sum::<usize>();

    Ok(result as u32)
}

fn corner_count(n: &(i32, i32), map: &HashMap<(i32, i32), char>, group_id: &char) -> usize {
    let (n_x, n_y) = n;
    let mut count = 0;
    for ([x, y], [x1, y1]) in DIRECTIONS.iter().circular_tuple_windows() {
        let dir_a = map.get(&(x + n_x, y + n_y)).is_some_and(|c| c == group_id);
        let dir_b = map
            .get(&(x1 + n_x, y1 + n_y))
            .is_some_and(|c| c == group_id);

        if dir_a
            && dir_b
            && map
                .get(&(x + x1 + n_x, y + y1 + n_y))
                .is_some_and(|c| c != group_id)
        {
            // interior corner, checks the diagonal where the * is
            // X*
            // XX
            count += 1;
        } else if !dir_a && !dir_b {
            // exterior corner
            // XX
            // *X
            count += 1;
        }
    }
    count
}

#[cfg(test)]
mod day_12_part2 {
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
        assert_eq!(1206, process(input)?);
        Ok(())
    }

    #[test_log::test]
    fn input() -> miette::Result<()> {
        let input = include_str!("../inputs/input.txt");
        assert_eq!(815788, process(input)?);
        Ok(())
    }
}
