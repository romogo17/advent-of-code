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
use tracing::debug;

fn parse(input: &str) -> IResult<&str, Vec<(&str, &str)>> {
    let (input, list) =
        separated_list1(line_ending, separated_pair(alpha1, tag("-"), alpha1))(input)?;
    let (input, _) = opt(line_ending)(input)?;

    Ok((input, list))
}

#[tracing::instrument(skip(input))]
pub fn process(input: &str, expected_cluster_size: usize) -> miette::Result<String> {
    let (_input, data) = all_consuming(parse)(input).map_err(|e| miette!("parse failed {}", e))?;

    let g = &UnGraphMap::<&str, ()>::from_edges(&data);

    let result = g
        .nodes()
        .flat_map(|node| {
            g.neighbors(node)
                .combinations(expected_cluster_size)
                .filter_map(move |neighbor_subset| {
                    if neighbor_subset
                        .iter()
                        .tuple_combinations()
                        .all(move |(a, b)| g.contains_edge(a, b))
                    {
                        let mut nodes = vec![node]
                            .into_iter()
                            .chain(neighbor_subset.into_iter())
                            .collect::<Vec<_>>();
                        nodes.sort();
                        Some(nodes)
                    } else {
                        None
                    }
                })
        })
        .unique()
        .collect::<Vec<_>>();

    if result.len() != 1 {
        debug!(?result);
        return Err(miette!("expected 1 result, got {}", result.len()));
    }

    Ok(result[0].iter().join(","))
}

#[cfg(test)]
mod day_23_part2 {
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
        assert_eq!("co,de,ka,ta", process(input, 3)?);
        Ok(())
    }

    #[test_log::test]
    fn input() -> miette::Result<()> {
        let input = include_str!("../inputs/input.txt");
        assert_eq!(
            "da,do,gx,ly,mb,ns,nt,pz,sc,si,tp,ul,vl",
            process(input, 12)?
        );
        Ok(())
    }
}
