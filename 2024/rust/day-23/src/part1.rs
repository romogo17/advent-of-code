use itertools::Itertools;
use miette::miette;
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, line_ending},
    combinator::{all_consuming, opt},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};
use petgraph::prelude::UnGraphMap;

fn parse(input: &str) -> IResult<&str, Vec<(&str, &str)>> {
    let (input, list) =
        separated_list1(line_ending, separated_pair(alpha1, tag("-"), alpha1))(input)?;
    let (input, _) = opt(line_ending)(input)?;

    Ok((input, list))
}

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<u32> {
    let (_input, data) = all_consuming(parse)(input).map_err(|e| miette!("parse failed {}", e))?;

    let g = &UnGraphMap::<&str, ()>::from_edges(&data);

    let result = g
        .nodes()
        .flat_map(|node| {
            g.neighbors(node)
                .tuple_combinations()
                .filter(move |(a, b)| {
                    g.contains_edge(a, b) && [node, a, b].iter().any(|n| n.starts_with("t"))
                })
                .map(move |(a, b)| {
                    let mut nodes = [node, a, b];
                    nodes.sort();
                    nodes
                })
        })
        .unique()
        .count() as u32;

    Ok(result)
}

#[cfg(test)]
mod day_23_part1 {
    use super::*;

    #[test_log::test]
    fn example() -> miette::Result<()> {
        let input = "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn";
        assert_eq!(7, process(input)?);
        Ok(())
    }

    #[test_log::test]
    fn input() -> miette::Result<()> {
        let input = include_str!("../inputs/input.txt");
        assert_eq!(1437, process(input)?);
        Ok(())
    }
}
