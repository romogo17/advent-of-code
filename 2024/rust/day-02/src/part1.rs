use crate::custom_error::AocError;
use itertools::Itertools;
use nom::{
    character::complete::{self, line_ending, multispace0, space1},
    multi::separated_list0,
    sequence::terminated,
    IResult,
};
use tracing::debug;

#[derive(Debug, PartialEq, Copy, Clone)]
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
    debug!(?report);
    let levels: Vec<(Order, u64)> = report
        .iter()
        .tuple_windows()
        .map(|(a, b)| match a.cmp(b) {
            std::cmp::Ordering::Less => (Order::Increasing, b - a),
            std::cmp::Ordering::Greater => (Order::Decreasing, a - b),
            std::cmp::Ordering::Equal => (Order::Flat, 0),
        })
        .collect();

    debug!(?levels);

    let initial_order = levels.first().unwrap().0;
    for (order, diff) in levels {
        match (order, diff) {
            (Order::Flat, _) => return false,
            (Order::Increasing, _) if initial_order == Order::Decreasing => return false,
            (Order::Decreasing, _) if initial_order == Order::Increasing => return false,
            (_, diff) if diff < 1 || diff > 3 => return false,
            _ => {}
        }
    }

    debug!("report is safe");
    true
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
mod day_02_part1 {
    use super::*;

    #[test_log::test]
    fn example() -> miette::Result<()> {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        assert_eq!(2, process(input)?);
        Ok(())
    }

    #[test_log::test]
    fn input() -> miette::Result<()> {
        let input = include_str!("../inputs/input.txt");
        assert_eq!(202, process(input)?);
        Ok(())
    }
}
