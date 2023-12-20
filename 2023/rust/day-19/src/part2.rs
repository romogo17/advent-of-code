use crate::custom_error::AocError;

use itertools::iproduct;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, alpha1, line_ending, multispace1},
    combinator::opt,
    multi::{fold_many1, separated_list1},
    sequence::{delimited, separated_pair, terminated},
    IResult, Parser,
};
use std::collections::{HashMap, HashSet};
use std::{cmp::Ordering, ops::RangeInclusive};
use tracing::{debug, info, warn};

// The puzzle uses only less than and greater than. Otherwise we would use Ordering
#[derive(Debug, PartialEq, Eq)]
enum Condition {
    GreaterThan,
    LessThan,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Target<'a> {
    Workflow(&'a str),
    Accepted,
    Rejected,
}

#[derive(Debug, PartialEq, Eq)]
enum Rule<'a> {
    Test {
        attribute: &'a str,
        condition: Condition,
        value: u64,
        target: Target<'a>,
    },
    Target(Target<'a>),
}

impl<'a> Rule<'a> {
    fn next_target(&self, part: &Part) -> Option<&Target> {
        match self {
            Rule::Test {
                attribute,
                condition,
                value,
                target,
            } => {
                let attribute_value = match *attribute {
                    "x" => part.x,
                    "m" => part.m,
                    "a" => part.a,
                    "s" => part.s,
                    _ => panic!("unknown attribute {}", attribute),
                };
                let ordering = match condition {
                    Condition::LessThan => Ordering::Less,
                    Condition::GreaterThan => Ordering::Greater,
                };
                (attribute_value.cmp(value) == ordering).then_some(target)
            }
            Rule::Target(target) => Some(target),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Workflow<'a> {
    id: &'a str,
    rules: Vec<Rule<'a>>,
}

#[derive(Debug, PartialEq, Eq, Default)]
struct Part {
    x: u64,
    m: u64,
    a: u64,
    s: u64,
}

impl Part {
    fn is_accepted(&self, workflows: &HashMap<&str, Workflow>) -> bool {
        let mut workflow_id = "in";
        let last_target: Target = 'workflow_loop: loop {
            let workflow = workflows.get(workflow_id).expect("should have a workflow");

            'rule_loop: for rule in workflow.rules.iter() {
                match rule.next_target(self) {
                    Some(Target::Accepted) => {
                        break 'workflow_loop Target::Accepted;
                    }
                    Some(Target::Rejected) => {
                        break 'workflow_loop Target::Rejected;
                    }
                    Some(Target::Workflow(next_workflow_id)) => {
                        workflow_id = next_workflow_id;
                        break 'rule_loop;
                    }
                    None => {}
                }
            }
        };
        match last_target {
            Target::Workflow(_) => panic!("should not have a workflow as last target"),
            Target::Accepted => true,
            Target::Rejected => false,
        }
    }
}

fn target(input: &str) -> IResult<&str, Target> {
    alt((
        tag("A").map(|_| Target::Accepted),
        tag("R").map(|_| Target::Rejected),
        alpha1.map(|workflow_id| Target::Workflow(workflow_id)),
    ))(input)
}

fn rule_test(input: &str) -> IResult<&str, Rule> {
    let (input, attribute) = alpha1(input)?;
    let (input, condition) = alt((
        tag("<").map(|_| Condition::LessThan),
        tag(">").map(|_| Condition::GreaterThan),
    ))(input)?;
    let (input, value) = complete::u64(input)?;
    let (input, _) = complete::char(':')(input)?;
    let (input, target) = target(input)?;
    Ok((
        input,
        Rule::Test {
            attribute,
            condition,
            value,
            target,
        },
    ))
}

fn workflow(input: &str) -> IResult<&str, Workflow> {
    let (input, id) = alpha1(input)?;
    let (input, rules) = delimited(
        complete::char('{'),
        separated_list1(
            complete::char(','),
            alt((rule_test, target.map(Rule::Target))),
        ),
        complete::char('}'),
    )(input)?;
    Ok((input, Workflow { id, rules }))
}

fn workflows(input: &str) -> IResult<&str, HashMap<&str, Workflow>> {
    let (input, workflows) = separated_list1(line_ending, workflow)(input)?;
    Ok((input, workflows.into_iter().map(|w| (w.id, w)).collect()))
}

fn part(input: &str) -> IResult<&str, Part> {
    delimited(
        complete::char('{'),
        fold_many1(
            terminated(
                separated_pair(alpha1, complete::char('='), complete::u64),
                opt(tag(",")),
            ),
            Part::default,
            |mut part, (next_attribute, count)| {
                match next_attribute {
                    "x" => part.x = count,
                    "m" => part.m = count,
                    "a" => part.a = count,
                    "s" => part.s = count,
                    _ => panic!("unknown attribute {}", next_attribute),
                }
                part
            },
        ),
        complete::char('}'),
    )(input)
}

fn parts(input: &str) -> IResult<&str, Vec<Part>> {
    separated_list1(line_ending, part)(input)
}

fn parse(input: &str) -> IResult<&str, (Vec<Part>, HashMap<&str, Workflow>)> {
    let (input, workflows) = workflows(input)?;
    let (input, _) = multispace1(input)?;
    let (input, parts) = parts(input)?;
    Ok((input, (parts, workflows)))
}

fn compute_ranges<'a>(
    workflows: &HashMap<&str, Workflow<'a>>,
) -> HashMap<&'a str, HashSet<RangeInclusive<u64>>> {
    let mut range_sets: HashMap<&'a str, HashSet<RangeInclusive<u64>>> = HashMap::from([
        ("a", HashSet::from([RangeInclusive::new(1, 4000)])),
        ("m", HashSet::from([RangeInclusive::new(1, 4000)])),
        ("s", HashSet::from([RangeInclusive::new(1, 4000)])),
        ("x", HashSet::from([RangeInclusive::new(1, 4000)])),
    ]);

    for (_id, workflow) in workflows.iter() {
        for rule in workflow.rules.iter() {
            match rule {
                Rule::Test {
                    attribute,
                    condition,
                    value,
                    target: _,
                } => {
                    let range_set = range_sets.get_mut(attribute).expect("should have a range");
                    let mut new_range_set: HashSet<RangeInclusive<u64>> = HashSet::new();

                    for range in range_set.iter() {
                        match condition {
                            // attr < value
                            Condition::LessThan => {
                                if range.contains(value) {
                                    new_range_set.insert(range.start().clone()..=*value - 1);
                                    new_range_set.insert(*value..=*range.end());
                                } else {
                                    new_range_set.insert(range.clone());
                                }
                            }
                            // attr > value
                            Condition::GreaterThan => {
                                if range.contains(value) {
                                    new_range_set.insert(range.start().clone()..=*value);
                                    new_range_set.insert(*value + 1..=*range.end());
                                } else {
                                    new_range_set.insert(range.clone());
                                }
                            }
                        }
                    }

                    *range_set = new_range_set;
                }
                Rule::Target(_) => {}
            }
        }
    }

    range_sets
}

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<u64, AocError> {
    let (_input, (_parts, workflows)) = parse(input).expect("should parse the puzzle input");

    let range_sets = compute_ranges(&workflows);
    debug!(?range_sets, "range sets");

    let mut sum = 0u64;
    iproduct!(
        range_sets.get("x").expect("should have a range"),
        range_sets.get("m").expect("should have a range"),
        range_sets.get("a").expect("should have a range"),
        range_sets.get("s").expect("should have a range")
    )
    .for_each(|(x, m, a, s)| {
        debug!(?x, ?m, ?a, ?s, "checking parts with attributes in");
        let sample_part = Part {
            x: x.start().clone(),
            m: m.start().clone(),
            a: a.start().clone(),
            s: s.start().clone(),
        };
        match sample_part.is_accepted(&workflows) {
            true => {
                // info!(?x, ?m, ?a, ?s, "found a part that is accepted");
                sum +=
                    (x.clone().count() * m.clone().count() * a.clone().count() * s.clone().count())
                        as u64;
            }
            false => {}
        }
    });

    Ok(sum)
}

#[cfg(test)]
mod day_19_part2 {
    use super::*;

    #[test_log::test]
    fn workflow_parser() {
        let input = "px{a<2006:qkq,m>2090:A,rfg}";
        assert_eq!(
            Workflow {
                id: "px",
                rules: vec![
                    Rule::Test {
                        attribute: "a",
                        condition: Condition::LessThan,
                        value: 2006,
                        target: Target::Workflow("qkq")
                    },
                    Rule::Test {
                        attribute: "m",
                        condition: Condition::GreaterThan,
                        value: 2090,
                        target: Target::Accepted
                    },
                    Rule::Target(Target::Workflow("rfg"))
                ]
            },
            workflow(input).expect("should parse a workflow").1
        )
    }

    #[test_log::test]
    fn example() -> miette::Result<()> {
        let input = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";
        assert_eq!(167409079868000, process(input)?);
        Ok(())
    }

    // #[test_log::test]
    // fn input1() -> miette::Result<()> {
    //     let input = include_str!("../inputs/input1.txt");
    //     assert_eq!(100, process(input)?);
    //     Ok(())
    // }
}
