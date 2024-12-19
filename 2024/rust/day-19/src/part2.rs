use cached::proc_macro::cached;
use miette::miette;
use nom::{
    bytes::complete::tag,
    character::complete::{self, alpha1, line_ending, multispace1},
    combinator::{all_consuming, opt},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};
use tracing::debug;

fn parse(input: &str) -> IResult<&str, (Vec<&str>, Vec<&str>)> {
    let (input, (towels, patterns)) = separated_pair(
        separated_list1(tag(", "), complete::alpha1),
        multispace1,
        separated_list1(line_ending, alpha1),
    )(input)?;
    let (input, _) = opt(line_ending)(input)?;

    Ok((input, (towels, patterns)))
}

#[cached(key = "String", convert = r##"{ format!("{pattern}") }"##)]
fn is_pattern_valid(pattern: &str, towels: &[&str]) -> usize {
    return towels
        .iter()
        .filter_map(|towel| {
            if pattern.starts_with(*towel) {
                let new_pattern = &pattern[towel.len()..];
                if new_pattern.is_empty() {
                    return Some(1);
                }
                Some(is_pattern_valid(new_pattern, towels))
            } else {
                None
            }
        })
        .sum();
}

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<u64> {
    let (_input, (towels, patterns)) =
        all_consuming(parse)(input).map_err(|e| miette!("parse failed {}", e))?;
    debug!(?towels, ?patterns);

    let count: usize = patterns.iter().map(|d| is_pattern_valid(d, &towels)).sum();
    Ok(count as u64)
}

#[cfg(test)]
mod day_19_part2 {
    use super::*;

    #[test_log::test]
    fn example() -> miette::Result<()> {
        let input = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";
        assert_eq!(16, process(input)?);
        Ok(())
    }

    #[test_log::test]
    fn input() -> miette::Result<()> {
        let input = include_str!("../inputs/input.txt");
        assert_eq!(601201576113503, process(input)?);
        Ok(())
    }
}
