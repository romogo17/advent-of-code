use std::collections::HashMap;

use crate::custom_error::AocError;
use itertools::Itertools;
use nom::{
    character::complete::{self, line_ending, multispace0, space1},
    multi::separated_list0,
    sequence::terminated,
    IResult,
};
use tracing::debug;

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
enum Order {
    Increasing,
    Decreasing,
    Flat,
}

fn parse(input: &str) -> IResult<&str, Vec<Vec<u64>>> {
    terminated(
        separated_list0(line_ending, separated_list0(space1, complete::u64)),
        multispace0,
    )(input)
}

fn is_report_safe(report: &Vec<u64>) -> bool {
    _is_report_safe(report, 0)
}

fn _is_report_safe(report: &Vec<u64>, recursion_level: usize) -> bool {
    let indent = "=>    ".repeat(recursion_level);
    if recursion_level > 1 {
        // indent debug message with a space for each recursion level

        debug!("{}recursion level exceeded, NOT SAFE", indent);
        return false;
    }

    debug!("{}report: {:?}", indent, report);
    let levels: Vec<(Order, u64)> = report
        .iter()
        .tuple_windows()
        .map(|(a, b)| match a.cmp(b) {
            std::cmp::Ordering::Less => (Order::Increasing, b - a),
            std::cmp::Ordering::Greater => (Order::Decreasing, a - b),
            std::cmp::Ordering::Equal => (Order::Flat, 0),
        })
        .collect();
    debug!("{}levels: {:?}", indent, levels);

    let order_freqs = levels.iter().fold(HashMap::new(), |mut map, (order, _)| {
        map.entry(order).and_modify(|freq| *freq += 1).or_insert(1);
        map
    });
    debug!("{}order_freqs={:?}", indent, order_freqs);

    // we can tolerate a single bad level, so there can be at most 2 orders and only one of them can have a frequency > 1
    if order_freqs.len() > 2 || order_freqs.values().filter(|&&v| v > 1).count() != 1 {
        debug!("{}multiple orders with frequency > 1, NOT SAFE", indent);
        return false;
    }

    // get most popular order
    let (most_popular_order, _) = order_freqs.iter().max_by_key(|(_, &freq)| freq).unwrap();

    // record the positions that are not safe, which are the positions that are either not following the most popular order or have a diff outside of the range [1, 3]
    let unsafe_positions: Vec<usize> = levels
        .iter()
        .enumerate()
        .filter_map(|(i, (order, diff))| match (order, diff) {
            (o, _) if o != *most_popular_order => Some(vec![i, i + 1]),
            (_, d) if *d < 1 || *d > 3 => Some(vec![i, i + 1]),
            _ => None,
        })
        .flatten()
        .collect();
    debug!("{}unsafe positions: {:?}", indent, unsafe_positions);

    // if there's no unsafe positions, we're good
    if unsafe_positions.len() == 0 {
        debug!("{}report is SAFE", indent);
        return true;
    }

    // otherwise, take each one of the unsafe positions and check if the report is safe by removing any of them
    for unsafe_position in unsafe_positions.iter() {
        let mut report = report.clone();
        report.remove(*unsafe_position);
        debug!("{}checking without position {}", indent, unsafe_position);
        if _is_report_safe(&report, recursion_level + 1) {
            return true;
        }
    }

    debug!("{}default case, NOT SAFE", indent);
    false
}

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<u64, AocError> {
    let (_, reports) = parse(input).expect("should parse input");
    debug!(?reports);

    let safe_reports = reports
        .into_iter()
        .filter(|r| r.len() > 0 && is_report_safe(r))
        .count() as u64;
    Ok(safe_reports)
}

#[cfg(test)]
mod day_02_part2 {
    use super::*;

    #[test_log::test]
    fn example1() -> miette::Result<()> {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        assert_eq!(4, process(input)?);
        Ok(())
    }

    #[test_log::test]
    fn example2() -> miette::Result<()> {
        let input = "1 6 7 8 9
6 1 7 8 9
14 15 14 16 17";
        assert_eq!(3, process(input)?);
        Ok(())
    }

    #[test_log::test]
    fn input() -> miette::Result<()> {
        let input = include_str!("../inputs/input.txt");
        assert_eq!(271, process(input)?);
        Ok(())
    }
}
