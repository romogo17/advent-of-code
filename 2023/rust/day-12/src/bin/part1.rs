use day_12::*;

use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, multispace0, space1},
    combinator::all_consuming,
    multi::{many1, separated_list1},
    sequence::{separated_pair, terminated},
    IResult, Parser,
};

fn parse_spring_condition_records(input: &str) -> IResult<&str, Vec<(SpringsLine, Vec<u64>)>> {
    let (input, output) = all_consuming(many1(terminated(
        separated_pair(
            many1(alt((
                tag("#").map(|_| Condition::Damaged),
                tag(".").map(|_| Condition::Operational),
                tag("?").map(|_| Condition::Unknown),
            )))
            .map(|conditions| SpringsLine::new(conditions)),
            space1,
            separated_list1(tag(","), complete::u64),
        ),
        multispace0,
    )))(input)?;

    Ok((input, output))
}

fn calculate_arrangements(springs_line: &SpringsLine, groups: &Vec<u64>) -> u64 {
    let missing_conditions = springs_line.count_missing();

    // println!(
    //     "==> processing '{}', which has groups {:?} and {} missing spring conditions",
    //     springs_line, groups, missing_conditions
    // );

    let permutations = itertools::repeat_n(
        [Condition::Damaged, Condition::Operational].iter(),
        missing_conditions,
    )
    .multi_cartesian_product();

    permutations
        .filter(|permutation| {
            // println!("permutation: {:?}", permutation);
            let candidate = SpringsLine::from_missing_permutation(&springs_line, permutation);
            let candidate_groups = candidate.count_damaged_group_lengths();
            let is_valid = candidate_groups == *groups;
            // println!(
            //     "candidate: '{}' with groups {:?}, valid={}",
            //     candidate, candidate_groups, is_valid
            // );
            is_valid
        })
        .count() as u64
}

fn process(input: &str) -> u64 {
    let (_, records) = parse_spring_condition_records(input).expect("should parse a space grid");

    records
        .iter()
        .map(|(conditions, damaged_springs)| {
            let valid_arrangements = calculate_arrangements(conditions, damaged_springs);
            // println!(
            //     "'{}' has {} valid arrangements",
            //     conditions, valid_arrangements
            // );
            valid_arrangements
        })
        .sum()
}

fn main() {
    let input = include_str!("../../inputs/input1.txt");
    let output = process(input);
    println!("Output is {output}");
}

#[cfg(test)]
mod day_12_part1 {
    use super::*;

    #[test]
    fn example() {
        let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
        let output = process(input);
        assert_eq!(output, 21);
    }

    #[test]
    fn input1() {
        let input = include_str!("../../inputs/input1.txt");
        let output = process(input);
        assert_eq!(output, 7191);
    }
}
